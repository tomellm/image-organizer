import os
from django.http import JsonResponse
from django.http import HttpResponse
from django.template import loader


def index(request):
    num_files = len(os.listdir(os.path.abspath("images/to_org")))
    data = {
        'num_files': num_files,
    }
    return JsonResponse(data)

def image(request, img_num):
    files = os.listdir(os.path.abspath("images/to_org"))
    if img_num < len(files):
        return JsonResponse({'succesful' : 'true', 'img_name' : files[img_num]})
    else:
        return JsonResponse({'succesful' : 'false'})