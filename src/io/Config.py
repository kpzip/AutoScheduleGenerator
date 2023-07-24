'''
Created on Jul 24, 2023

@author: kpzip
'''
import json

class Config:

    def __init__(self, version_num):
        self.version = version_num

    def getVersion(self):
        return self.version

def readConfig(location):
    configfile = open(location, "r")
    configdata = json.load(configfile)
    configfile.close()
    configobject = Config(configdata["version"])
    return configobject

CONFIG = readConfig("settings.json")
print (CONFIG.getVersion())


    