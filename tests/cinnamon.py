from direct.directbase.DirectStart import base

import adopt_a_doodle

cinnamon = adopt_a_doodle.adopt(adopt_a_doodle.cinnamon())
cinnamon.setPos(0,5,-1.2)
cinnamon.setH(180)
cinnamon.reparentTo(render)

base.run()