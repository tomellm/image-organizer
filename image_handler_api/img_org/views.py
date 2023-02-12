from pathlib import Path
from django.http import JsonResponse
from django.http import HttpResponse
from django.template import loader
from django.conf import settings


root_path = settings.BASE_DIR 
img_to_org_path = Path.joinpath(root_path, "images/to_org")

def get_to_org_list():
    return list(img_to_org_path.iterdir())

def index(request):
    file_list = [str(x.name) for x in img_to_org_path.iterdir()]
    return JsonResponse({
        'num_files' : len(get_to_org_list()),
        'all_files' : file_list,
        'images_base_path' : str(img_to_org_path)
    })

def image(request, img_num):
    files = get_to_org_list()
    if img_num < len(files):
        return JsonResponse({
            'succesful' : 'true', 
            'img_name' : files[img_num].name,
            'abs_img_path' : str(files[img_num])
        })
    else:
        return JsonResponse({'succesful' : 'false'})