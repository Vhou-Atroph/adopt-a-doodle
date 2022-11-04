"""Test module creating an instance of Cinnamon"""
from panda3d.core import *
from direct.directbase import DirectStart

import adopt
import rustydoodle

cinnamon = adopt.adopt(rustydoodle.cinnamon())
cinnamon.reparentTo(render)

base.run()