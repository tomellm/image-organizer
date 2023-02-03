"""
    This code comes from:
        - https://github.com/mpapazog/heic-to-jpg/blob/master/heictojpg.py
"""

import os, subprocess

def convert_elements_to_jpg(path, delete_original=False):
    for filename in os.listdir(path):
        if filename.lower().endswith(".heic"):
            subprocess.run(["magick", "%s" % (path + filename), "%s" % (path + filename[0:-5] + '.jpg')])
            if delete_original:
                os.remove(path + filename)
            continue

#convert_elements_to_jpg('images/editable_media/', delete_original=True)