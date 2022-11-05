from direct.actor.Actor import Actor
from direct.showbase.ShowBase import ShowBase  
from pandac import *
from direct import *
from panda3d.core import *
from direct.showbase.Loader import *

from . import rustydoodle

def adopt(actor:rustydoodle.Doodle) -> Actor:
    """Creates a Doodle actor."""
    doodle = Actor("TT_pets-mod.bam",{"animation":actor.animation.file})

    doodle.findAllMatches('**/body').setTexture(loader.loadTexture(actor.pattern.body),1)
    doodle.findAllMatches('**/body').setColor(actor.color)
    doodle.findAllMatches('**/rightFoot').setTexture(loader.loadTexture(actor.pattern.legs),1)
    doodle.findAllMatches('**/leftFoot').setTexture(loader.loadTexture(actor.pattern.legs),1)
    doodle.findAllMatches('**/rightFoot').setColor(actor.color)
    doodle.findAllMatches('**/leftFoot').setColor(actor.color)
    if actor.pattern.ears:
        doodle.findAllMatches('**/'+actor.ears).setTexture(loader.loadTexture(actor.pattern.ears),1)
    if actor.pattern.tail:
        doodle.findAllMatches('**/'+actor.tail).setTexture(loader.loadTexture(actor.pattern.tail),1)
    if not actor.eyelashes:
        doodle.findAllMatches('**/eyeWhites').setTexture(loader.loadTexture('phase_4/maps/BeanEyeBoys2.jpg','phase_4/maps/BeanEyeBoys2_a.rgb'),1)
    doodle.findAllMatches('**/eyeWhites').setColor(1,1,1,1)
    doodle.findAllMatches('**/rightPupil').setColor(actor.eye_color)
    doodle.findAllMatches('**/leftPupil').setColor(actor.eye_color)
    doodle.findAllMatches('**/rightBrow').setColor(0,0,0,1)
    doodle.findAllMatches('**/leftBrow').setColor(0,0,0,1)

    _getparts(actor,doodle,rustydoodle.hair_list(),actor.hair)
    _getparts(actor,doodle,rustydoodle.ear_list(),actor.ears)
    _getparts(actor,doodle,rustydoodle.nose_list(),actor.nose)
    _getparts(actor,doodle,rustydoodle.tail_list(),actor.tail)

    doodle.play("animation")
    if actor.animation.anim_loop:
        doodle.loop("animation",restart=actor.animation.loop_restart,fromFrame=actor.animation.loop_from,toFrame=actor.animation.loop_to)
    if actor.animation.pose:
        doodle.pose("animation",actor.pose_frame)
    return doodle

def _getparts(actor:rustydoodle.Doodle,doodle:Actor,part_list:list,part:str):
    for i in part_list:
        if i!= part:
            doodle.findAllMatches('**/'+i).hide()
        else:
            doodle.findAllMatches('**/'+i).setColor(actor.color)