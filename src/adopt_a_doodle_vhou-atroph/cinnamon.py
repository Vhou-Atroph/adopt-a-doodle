"""Test module creating an instance of Cinnamon"""
from panda3d.core import *
from direct.directbase import DirectStart

import main

cinnamon = main.adopt(main.rustydoodle.cinnamon())
cinnamon.reparentTo(render)

base.run()