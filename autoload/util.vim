function! util#base64(s) abort
    if !has('python') && !has('python3')
        throw 'Need Python support'
    endif
    try
        if has('python3')
            python3 << EOF
import base64
import vim
s = vim.eval('a:s')
res = base64.b64encode(s.encode('utf-8')).decode('utf-8')
EOF
            return py3eval('res')
        else
            python << EOF
import base64
import vim
s = vim.eval('a:s')
res = base64.b64encode(s)
EOF
            return pyeval('res')
        endif
    catch /.*/
        throw 'Base64 Error: ' . v:exception
    endtry
endfunction

function! util#md5(s) abort
    if !has('python') && !has('python3')
        throw 'util#md5: Requires Vim with Python support'
    endif

    try
        if has('python3')
            python3 << EOF
import hashlib
import vim

try:
    s = vim.eval('a:s')
    md5 = hashlib.md5()
    md5.update(s.encode('utf-8'))
    res = md5.hexdigest()
except UnicodeEncodeError:
    res = hashlib.md5(s.encode('utf-8', errors='surrogateescape')).hexdigest()
EOF
            return py3eval('res')
        else
            python << EOF
import hashlib
import vim
s = vim.eval('a:s')
if isinstance(s, unicode):
    s = s.encode('utf-8')
res = hashlib.md5(s).hexdigest()
EOF
            return pyeval('res')
        endif
    catch /.*/
        throw 'MD5Error: ' . v:exception
    endtry
endfunction
