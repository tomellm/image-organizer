from django.urls import path

from . import views

urlpatterns = [
    path('', views.index, name='index'),
    path('<int:img_num>', views.image, name='image'),
]