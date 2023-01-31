import os, subprocess
from datetime import datetime
import pandas as pd
from libxmp import XMPFiles, consts
from tqdm import tqdm
import logging

class Logger:
    def _init_logger(self, name):
        class CustomFormatter(logging.Formatter):
            grey = "\x1b[38;20m"
            yellow = "\x1b[33;20m"
            red = "\x1b[31;20m"
            bold_red = "\x1b[31;1m"
            reset = "\x1b[0m"
            format = "%(levelname)s - %(message)s (%(filename)s:%(lineno)d)"

            FORMATS = {
                logging.DEBUG: grey + format + reset,
                logging.INFO: grey + format + reset,
                logging.WARNING: yellow + format + reset,
                logging.ERROR: red + format + reset,
                logging.CRITICAL: bold_red + format + reset
            }

            def format(self, record):
                log_fmt = self.FORMATS.get(record.levelno)
                formatter = logging.Formatter(log_fmt)
                return formatter.format(record)


        # create logger with 'spam_application'
        logger = logging.getLogger(name)
        logger.setLevel(logging.DEBUG)

        # create console handler with a higher log level
        ch = logging.StreamHandler()
        ch.setLevel(logging.DEBUG)
        ch.setFormatter(CustomFormatter())
        logger.addHandler(ch)
        return logger

class Organizer:
    
    def __init__(self, target_dir, dump_dir):
        self.logger = Logger()._init_logger("Image Organizer")
        self.tp = target_dir
        self.dp = dump_dir

        self.date_notations = [
            [consts.XMP_NS_XMP, 'CreateDate'],
            [consts.XMP_NS_EXIF, 'DateTimeOriginal'],
            [consts.XMP_NS_XMP, 'ModifyDate'],
            [consts.XMP_NS_Photoshop, 'DateCreated'] 
        ]
        self.date_format = '%Y-%m-%dT%H:%M'
        
        if self.tp[-1] != "/" and self.dp[-1] != "/":
            self.logger.error("Parameter paths need to end with a slash!")
            return None

    def dir_prepare(self):
        files = os.listdir(self.tp)
        self.logger.info("Cleaning up directory")
        for file in tqdm(files):
            if os.path.isdir(self.tp + file):
                if file == "heic":
                    self.logger.warning("Removed the heic directory!")
                    os.rmdir(self.tp + file)
                    continue
                else:
                    self.logger.error("There is a directory in targeted directory! please only have it contain media!")
                    return None
        columns = ["media", "xmp", "date-created", "new_path"]
        self.df = pd.DataFrame(columns=columns)
        
        self.hp = self.tp + "heic/"
        os.mkdir(self.hp)
        
        self.convert_elements_to_jpg()
        files = os.listdir(self.tp)

    def fill_df(self):
        for file in tqdm(os.listdir(self.tp)):
            if not os.path.isfile(self.tp + file):
                self.logger.warning(file + " is not a file in the folder")
                continue
            name, ext = os.path.splitext(file)
            if ext != ".xmp" and name[0] != '.':
                ret = self.get_xmp(self.tp + file)
                if ret:
                    xmp, date = ret[0], ret[1]
                else:
                    xmp, date = None, None
                self.df = pd.concat([self.df, pd.DataFrame([[file, xmp, date, None]], columns=self.df.columns)], ignore_index=True)

        self.df["date-created"] = self.df["date-created"].astype('datetime64[ns]')
        self.df.sort_values(by='date-created', inplace=True)
        self.df = self.df.reset_index()
        del self.df["index"]

    def save_to_new_paths(self):
        for path in self.paths_list:
            os.mkdir(self.dp + path)
        os.mkdir(self.dp + "nodate")

        self.logger.info("moving files to the dump directory")
        for index, row in tqdm(self.df.iterrows()):
            if row["media"]:
                if row["new_path"]:
                    os.rename(self.tp + row["media"], self.dp + row["new_path"] + row["media"])
                else:
                    os.rename(self.tp + row["media"], self.dp + "nodate/" + row["media"])

    def order_files(self, year=True, month=False, day=False, nested=True):
        print(year, month, day)
        if not year and not month and not day:
            self.logger.error("file ordering did not execute, please specify a measure to order by")
            return None

        self.logger.info("ordering files!")
        new_paths = set()
        depth = 1
        order = []
        
        if year:
            order += ["year"]
            depth += 1
        if month:
            order += ["month"]
            depth += 1
        if day:
            order += ["day"]
            depth += 1

        if nested:
            split_char = "/"
        else:
            split_char = "-"

        strc = {
            "year": {
                "active": year,
                "last": False
            },
            "month": {
                "active": month,
                "last": False
            },
            "day": {
                "active": day,
                "last": False
            }
        }

        for i in order[::-1]:
            if strc[i]["active"]:
                strc[i]["last"] = True
                break

        for index, row in tqdm(self.df.iterrows()):
            path = ""
            date = row["date-created"].to_pydatetime()
            if pd.isnull(date):
                continue
            for i in order:
                func = getattr(date, i)
                path += str(func)
                if not strc[i]["last"] or split_char == "/":
                    path += split_char
            new_paths.add(path)
            self.df.at[index, "new_path"] = path

        if nested:
            self.paths_list = self.create_list_paths(new_paths, depth)

    def create_list_paths(self, path_set, depth):
        path_array = []
        path_create_set = set()
        for path in path_set:
            path_array += [path.split("/")[0:-1]]

        for i in range(depth):
            for path in path_array:
                current = ""
                for j in range(i):
                    current += path[j] + "/"
                if current:
                    path_create_set.add(current)

        final_paths = []
        for i in range(depth):
            actual = i+1
            for path in list(path_create_set):
                if len(path.split("/")[0:-1]) == actual:
                    final_paths += [path]
            if len(final_paths) == len(path_create_set):
                break

        return final_paths

    def convert_elements_to_jpg(self, delete_original=False):
        self.logger.info("Converting Heics to jpgs")
        for filename in tqdm(os.listdir(self.tp)):
            if os.path.isdir(self.tp + filename):
                continue
            elif filename.lower().endswith(".heic"):
                subprocess.run(["magick", "%s" % (self.tp + filename), "%s" % (self.tp + filename[0:-5] + '.jpg')])
                if delete_original:
                    os.remove(self.tp + filename)
                else:
                    os.rename(self.tp + filename, self.hp + filename)
                continue

    def get_xmp(self, path_to_jpg):
        #if not path_to_jpg.lower().endswith(".jpg"):
        #    print("ERROR: passed File is not a JPG. XMP extraction will be cut short")
        #    return None
        xmpfile = XMPFiles(file_path=path_to_jpg, open_forupdate=False)
        xmp = xmpfile.get_xmp()
        if not xmp:
            self.logger.warning("WARNING: No xmp data was found")
            return None
        dates = []
        for date in self.date_notations:
            temp = xmp.get_property(date[0], date[1])
            if temp:
                temp = temp[0:16]
                dates += [datetime.strptime(temp, self.date_format)]
        if dates:
            dates = min(dates)
        else:
            dates = None
        return xmp, dates

class Organizer_Controller:
    def __init__(self, interal_mode=True):
        self.logger = Logger()._init_logger("Organizer Controller")
        self.mode = interal_mode
    
    def start(self):
        if not self.mode: 
            enter_val = True
            while enter_val:
                target_dir = input("Please enter the directory with the files: ")
                dump_dir = input("Please enter the directory where the files should end up: ")
                if target_dir[-1] == "/" and dump_dir[-1] == "/" and os.path.isdir(target_dir) and os.path.isdir(dump_dir):
                    self.info("Paths are correct")
                    enter_val = False
        else:
            target_dir = "working_files/original/"
            dump_dir = "working_files/dump/"

        self.org = Organizer(target_dir, dump_dir)

        self.org.dir_prepare()
        self.org.fill_df()

        self.org.order_files(month=True, day=True)
        self.org.save_to_new_paths()

Organizer_Controller().start()