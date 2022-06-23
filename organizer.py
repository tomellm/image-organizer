"""
    Used to make this project:
        - https://gist.github.com/nikomiko/7492e5e82791c9ff989e2573ca180273
"""
from cmath import e
import os
from shutil import ExecError
import time
import datetime
import re
from PIL import Image
from hachoir.parser import createParser
from hachoir.metadata import extractMetadata
from media_sorter import DATE_TIME_ORIG_TAG

original_images_path = "images/editable_media/"
target_path = "images/copy_to/"

re_string = r"\s\([0-9]+\)"

img_formats = [".jpg", ".jpeg", ".png"]
vid_formats = [".mp4", ".mov"]

num_cameras = {}


def get_dir_children(path):
    return os.listdir(path)


def get_date_taken(file_path):
    date_time_obj = ""
    filename, ext = os.path.splitext(file_path)
    if ext.lower() in img_formats:
        try:
            string_date = Image.open(file_path)._getexif()[36867]
            date_time_obj = datetime.datetime.strptime(string_date, "%Y:%m:%d %H:%M:%S")
        except Exception as err:
            string_date = time.ctime(os.path.getctime(file_path))
            date_time_obj = datetime.datetime.strptime(string_date, "%c")
    elif ext.lower() in vid_formats:
        parser = createParser(file_path)
        if not parser:
            print("Unable to parse file %s" % file_path)
        with parser:
            try:
                metadata = extractMetadata(parser)
            except Exception as err:
                print("Metadata extraction error: %s" % err)
                metadata = None
        if not metadata:
            print("Unable to extract metadata")
        for line in metadata.exportPlaintext():
            if line.split(':')[0] == '- Creation date':
                date_time_obj = datetime.datetime.strptime(
                    line.split(':')[1].split()[0], "%Y-%m-%d")
    else:
        string_date = time.ctime(os.path.getctime(file_path))
        date_time_obj = datetime.datetime.strptime(string_date, "%c")
    return date_time_obj.strftime('%y-%m-%d_%H:%M')


def get_camera_taken(path):
    try:
        string_camera = Image.open(path)._getexif()[272]
    except Exception as err:
        print(path)
        string_camera = "UNKOWN"
    return string_camera
    

def get_extension(path):
    return os.path.splitext(path)


def set_files_names(path, new_loc):
    images = get_dir_children(path)
    for image in images:
        if re.findall(re_string, image):
            original_file = re.sub(re_string, '', image)
            for new_image in images:
                if new_image == original_file:
                    os.remove(path + image)
                    break
            continue
        actual_name = path + image
        date_string = get_date_taken(actual_name).split('_')
        if not os.path.isdir(new_loc + date_string[0]):
            os.mkdir(new_loc + date_string[0])
        date_string = date_string[0] + "/" + date_string[1]
        os.rename(
            actual_name, 
            new_loc + date_string + image)


def fix_camera_names(camera):
    if camera == "EX-Z85":
        return "Casio EXILIM"
    elif camera == "ILCE-7M3":
        return "A7III"
    elif camera == "Canon EOS R":
        return "EOSR"
    elif camera == "Canon DIGITAL IXUS 70":
        return "IXUS70"
    elif camera == "GR II\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00":
        return "UNKOWN"
    else: return camera


def get_cameras_taken(target_path):
    for image in get_dir_children(target_path):
        filename, ext = os.path.splitext(image)
        if ext.lower() in img_formats:
            camera = get_camera_taken(target_path + image)
            camera = fix_camera_names(camera)
            if not camera in num_cameras:
                num_cameras[camera] = 0
            num_cameras[camera] = num_cameras[camera] + 1

set_files_names(original_images_path, target_path)