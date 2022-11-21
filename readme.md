# adopt-a-doodle

A Python package to make the creation of Doodle actors easier!

## Dependencies

adopt_a_doodle depends on Panda3D. If you have not installed it already, you can do so with the following command:

```ps
pip install Panda3D
```

## Installing

To install the latest version of adopt_a_doodle, open your favorite command terminal and use the following command:

```ps
pip install adopt_a_doodle
```

If for whatever reason you are unable to install adopt_a_doodle through pip, you can also install it through the [latest source distribution released on GitHub](https://github.com/Vhou-Atroph/adopt-a-doodle/releases/latest). Download the tar.gz file and open your favorite command terminal. Navigate to wherever the file was downloaded and run the following command:

```ps
pip install [file]
```

## Usage

With adopt_a_doodle, the creation of Doodle actors becomes much easier.  
Like with any other Panda3D Toontown project, you must first extract the Phase Files. You can do so with the following command, with [x] being replaced by the id of the phase file:

```ps
multify.exe -xf phase_[x].mf
```

With adopt_a_doodle, you will need phase_4, phase_5, and phase_5.5. Once these files are extracted, drop them into the same directory you want to have your Python files in. Your directory should look similar to this:

```none
| phase_4
| phase_5
| phase_5.5
| example_doodle.py
```

Next, go into phase_4/models and find TT_pets-mod.bam. This is the model file for doodles. Drop this file into your main working directory, which should now look like this:

```none
| phase_4
| phase_5
| phase_5.5
| example_doodle.py
| TT_pets-mod.bam
```

Now that all the necessary files are here, you can open the Python file containing your scene and start to program! Here's an example scene:

```python
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
```

This code will produce the following doodle:

![A purple doodle with Leopard spots, cat ears, and a long tail.](https://raw.githubusercontent.com/Vhou-Atroph/adopt-a-doodle/main/tests/example_doodle.png)

## Documentation

You can find documentation for adopt_a_doodle in the [rustydoodle lib.rs file](https://raw.githubusercontent.com/Vhou-Atroph/adopt-a-doodle/main/src/lib.rs).

## License

Code in adopt_a_doodle is licensed under the [MIT License](/LICENSE).
