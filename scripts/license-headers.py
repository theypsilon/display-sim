#!/usr/bin/env python
# updates the copyright information for all .cs files
# usage: call recursive_traversal, with the following parameters
# parent directory, old copyright text content, new copyright text content

import os

def update_source(filename, license_text):
    utfstr = chr(0xef)+chr(0xbb)+chr(0xbf)
    fdata = file(filename,"r+").read()
    isUTF = False
    if (fdata.startswith(utfstr)):
        isUTF = True
        fdata = fdata[3:]
    if not (fdata.startswith("/* Copyright")):
        print "updating "+filename
        fdata = license_text + fdata
        if (isUTF):
            file(filename,"w").write(utfstr+fdata)
        else:
            file(filename,"w").write(fdata)

def recursive_traversal(dir, extension, license_text):
    fns = os.listdir(dir)
    print "listing "+dir
    for fn in fns:
        fullfn = os.path.join(dir,fn)
        if (os.path.isdir(fullfn)):
            recursive_traversal(fullfn, extension, license_text)
        else:
            if (fullfn.endswith(extension)):
                update_source(fullfn, license_text)

license_text = ""
with open("LICENSE") as f:
    firstline = True
    for line in f.readlines():
        if firstline:
            license_text += "/* "
            firstline = False
        else:
            license_text += " * "
        license_text += line
    license_text += " */\n"

recursive_traversal("./rust", ".rs", license_text)