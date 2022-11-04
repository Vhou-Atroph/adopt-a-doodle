from direct.actor.Actor import Actor

from rustydoodle import Doodle,Color,Pattern,Animation

def adopt(actor:Doodle) -> Actor:
    """Creates a Doodle actor."""
    doodle = Actor("TT_pets-mod.bam",{"animation":actor.animation.file})

    doodle.play("animation")
    if actor.animation.anim_loop:
        doodle.loop("animation",restart=actor.animation.loop_restart,fromFrame=actor.animation.loop_from,toFrame=actor.animation.loop_to)
    return doodle