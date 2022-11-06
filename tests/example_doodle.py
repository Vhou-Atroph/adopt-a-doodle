from direct.directbase.DirectStart import base
import adopt_a_doodle

example_doodle = adopt_a_doodle.adopt(adopt_a_doodle.Doodle(
    color=(0.546875, 0.28125, 0.75, 1.0),
    eye_color=(0.242188, 0.742188, 0.515625, 1.0),
    pattern=adopt_a_doodle.Pattern(ears="phase_4/maps/BeanCatEar3Yellow.jpg",
        body="phase_4/maps/BeanbodyLepord2.jpg",
        legs="phase_4/maps/BeanFootYellow1.jpg",
        tail="phase_4/maps/BeanLongTailLepord.jpg"),
    animation=adopt_a_doodle.Animation(file="phase_5/models/char/TT_pets-speak.bam",
        anim_loop=True,
        loop_from=None,
        loop_to=None,
        loop_restart=None,
        pose=False,
        pose_frame=None),
    eyelashes=False,
    hair=None,
    ears="catEars",
    nose=None,
    tail="longTail"))
example_doodle.setPos(0,5,-1.2)
example_doodle.setH(180)
example_doodle.reparentTo(render)

base.run()
