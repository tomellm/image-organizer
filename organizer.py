"""
    Used to make this project:
        - https://gist.github.com/nikomiko/7492e5e82791c9ff989e2573ca180273
"""
import os
import time
import datetime
import re
import pandas as pd
from PIL import Image
from hachoir.parser import createParser
from hachoir.metadata import extractMetadata

original_images_path = "images/editable_media/"
target_path = "images/copy_to/"

last_slash_re = r"\/$"
double_copy_re = r"\s\([0-9]+\)"

img_formats = [".jpg", ".jpeg", ".png"]
vid_formats = [".mp4", ".mov"]

num_cameras = {}


def g_sub(path):
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
    return date_time_obj.strftime('%Y-%B'), date_time_obj.strftime('%d_%H_%M')

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

def get_camera_taken(path):
    try:
        string_camera = Image.open(path)._getexif()[272]
    except Exception as err:
        string_camera = "UNKOWN"
    return fix_camera_names(string_camera)

def get_extension(path):
    return os.path.splitext(path)

def get_cameras_taken(target_path):
    for image in g_sub(target_path):
        filename, ext = os.path.splitext(image)
        if ext.lower() in img_formats:
            camera = get_camera_taken(target_path + image)
            camera = fix_camera_names(camera)
            if not camera in num_cameras:
                num_cameras[camera] = 0
            num_cameras[camera] = num_cameras[camera] + 1


def get_media_info(path):
    image_info_array = pd.DataFrame()
    for image in g_sub(path):
        year_month, day_hour_min = get_date_taken(path + image)
        os.rename(
            path + image, 
            path + day_hour_min + "_" + image
        )
        image_info_array = pd.concat(
            [
                image_info_array,
                pd.DataFrame({
                    "name": [day_hour_min + "_" + image],
                    "year-month": [year_month],
                    "day_hour_min": [day_hour_min],
                    "camera": [get_camera_taken(path + image)]
                })
            ],
            ignore_index=True
        )
    return image_info_array

def set_files_names(path, new_loc):
    if not re.findall(last_slash_re, path):
        path = path + "/"
    if not re.findall(last_slash_re, new_loc):
        new_loc = new_loc + "/"
    if not os.path.isdir(new_loc + "deleted/"):
            os.mkdir(new_loc + "deleted/")
    media_df = get_media_info(path)
    for index, image in media_df.iterrows():
        actual_find = False
        if re.findall(double_copy_re, image):
            print("\n\nChecking the image: " + image)
            original_name = re.sub(double_copy_re, '', image)
            if(original_name == "lp_image.jpg"):
                continue
            elif(original_name == "lp_image.mov"):
                continue
            for new_image in images:
                if new_image == original_name:
                    print("\tFound partner: " + new_image)
                    print("\t\t" + str(os.path.exists(path + new_image)))
                    img1Y, img1D = get_date_taken(path + new_image)
                    img2Y, img2D = get_date_taken(path + image)
                    if img2D == img1D:
                        actual_find = True
                        #os.rename(path + image, new_loc + "deleted/" + image)
                        break
            if actual_find:
                continue
        actual_name = path + image
        date_string, time_string = get_date_taken(actual_name)
        copy_to_location = new_loc + date_string + "/"
        if not os.path.isdir(copy_to_location):
            os.mkdir(copy_to_location)
        os.rename(
            actual_name, 
            copy_to_location + time_string + "_" + image
        )





#set_files_names(original_images_path, target_path)