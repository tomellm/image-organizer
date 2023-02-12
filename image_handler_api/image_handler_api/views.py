from django.http import HttpResponse
from django.template import loader

def index(request):
    template = loader.get_template('image_handler_api/index.html')
    context = {
        'link_list': [
            {
                'link' : '/img_org',
                'desc' : 'gets number of files'
            },
            {
                'link' : '/img_org/<number>',
                'desc' : 'get a specific file in that list of files'
            }
        ],
    }
    return HttpResponse(template.render(context, request))