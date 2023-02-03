from django.contrib import admin

from .models import CameraModel, File

admin.site.register(CameraModel)
admin.site.register(File)
