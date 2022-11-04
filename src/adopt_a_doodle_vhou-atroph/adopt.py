from direct.actor.Actor import Actor

import rustydoodle

def adopt(actor:rustydoodle.Doodle) -> Actor:
    """Creates a Doodle actor."""
    doodle = Actor("TT_pets-mod.bam",{"animation":actor.animation.file})

    for i in rustydoodle.hair_list():
        if i != actor.hair:
            doodle.findAllMatches('**/'+i).hide()
    
    for i in rustydoodle.ear_list():
        if i != actor.ears:
            doodle.findAllMatches('**/'+i).hide()
    
    for i in rustydoodle.nose_list():
        if i != actor.nose:
            doodle.findAllMatches('**/'+i).hide()
    
    for i in rustydoodle.tail_list():
        if i != actor.tail:
            doodle.findAllMatches('**/'+i).hide()

    doodle.play("animation")
    if actor.animation.anim_loop:
        doodle.loop("animation",restart=actor.animation.loop_restart,fromFrame=actor.animation.loop_from,toFrame=actor.animation.loop_to)
    if actor.animation.pose:
        doodle.pose("animation",actor.pose_frame)
    return doodle