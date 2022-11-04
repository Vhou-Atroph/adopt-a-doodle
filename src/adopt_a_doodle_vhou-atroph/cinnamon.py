"""Test module creating an instance of Cinnamon"""
from panda3d.core import NodePath
from direct.directbase.DirectStart import base

import adopt
import rustydoodle

cinnamon = adopt.adopt(rustydoodle.cinnamon())
cinnamon.reparentTo(render)

base.run()