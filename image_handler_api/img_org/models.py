import os
from django.conf import settings
from django.db import models
from django.utils.translation import gettext_lazy as _



class CameraModel(models.Model):
    
    class CameraBrand(models.TextChoices):
        CANON = 'CANON', _('Canon')
        SONY = 'SONY', _('Sony')
        CASIO = 'Casio', _('Casio')

    model_name = models.CharField(max_length=20)
    camera_brand = models.CharField(
        max_length=10,
        choices=CameraBrand.choices
        )

    def __str__(self):
        return self.camera_brand + " " + self.model_name

class File(models.Model):
    class FileType(models.TextChoices):
        JPG = 'jpg', _('jpg')
        JPEG = 'jpeg', _('jpeg')
        PNG = 'png', _('png')
        HEIC = 'heic', _('heic')
        MP4 = 'mp4', _('mp4')
        MOV = 'mov', _('mov')
    
    
    file_path = models.FilePathField(path=os.path.abspath("images"))
    datetime_taken = models.DateTimeField()
    file_type = models.CharField(
        max_length=5,
        choices=FileType.choices
        )
    camera_taken = models.ForeignKey(
        CameraModel,
        null=True,
        on_delete=models.SET_NULL
    )

    def is_image(self):
        return self.file_type in [
            self.FileType.JPG,
            self.FileType.HEIC
        ]

    def is_video(self):
        return self.file_type in [
            self.FileType.MP4,
            self.FileType.MOV
        ]
    
    def __str__(self):
        return self.file_type + " taken on " + str(self.datetime_taken) + " with name " + str(self.file_path)




