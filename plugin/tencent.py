import base64
import json
from sys import argv, stdout
from urllib import error, request

headers = {
    'accept': 'application/json, text/plain, */*',
    'accept-language': 'zh-CN,zh;q=0.9',
    'content-type': 'application/json',
    'dnt': '1',
    'origin': 'https://transmart.qq.com',
    'priority': 'u=1, i',
    'referer': 'https://transmart.qq.com/zh-CN/index',
    'sec-ch-ua': '"Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128"',
    'sec-ch-ua-mobile': '?0',
    'sec-ch-ua-platform': '"macOS"',
    'sec-fetch-dest': 'empty',
    'sec-fetch-mode': 'cors',
    'sec-fetch-site': 'same-origin',
    'user-agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36',
    'x-requested-with': 'XMLHttpRequest',
}


URL = 'https://transmart.qq.com/api/imt'


def get_result(query):
    query = base64.b64decode(query).decode('utf-8')
    return _get_result(query)


def _get_result(query):
    data = {
        'header': {
            'fn': 'auto_translation',
            'session': '',
            'client_key': 'browser-chrome-128.0.0-Mac_OS-2276e164-8ba1-4597-a7f7-b0dd07e9de2d-1741855790540',
            'user': '',
        },
        'type': 'plain',
        'model_category': 'normal',
        'text_domain': '',
        'source': {
            'text_list': [query],
        },
    }

    try:
        data = json.dumps(data).encode('utf-8')
        req = request.Request(URL, data=data, headers=headers)
        res = request.urlopen(req)
        if res.getcode() != 200:
            return 'Err:返回异常[{}]'.format(res.getcode())
        res = json.loads(res.read())
        if 'auto_translation' not in res:
            return 'Err:返回异常'
        return '\n'.join(res['auto_translation'])
    except error.HTTPError:
        return 'Err:请求异常'
    except Exception as e:
        return 'Err:产生异常: %s' % e


if __name__ == '__main__':
    if len(argv) >= 2:
        stdout.write(str(get_result(argv[1])))
    stdout.flush()
