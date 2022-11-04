from direct.actor.Actor import Actor

import rustydoodle

def adopt(actor:rustydoodle.Doodle) -> Actor:
    doodle = Actor("TT_pets-mod.bam",{"animation":actor.animation.file})
    return doodle