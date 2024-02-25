
from typing import List

from bmd import SpeedEditorHandler, SpeedEditor

class DemoHandler(SpeedEditorHandler):
	def __init__(self, se):
		self.se   = se
        # clear the leds
		self.se.set_leds(0)
		self.se.set_jog_leds(0)


if __name__ == '__main__':
    se = SpeedEditor()
    se.authenticate()
    se.set_handler(DemoHandler(se))
    import random
    import time
    while True:
        time.sleep(1)
        se.set_leds(random.getrandbits(18))
        se.set_jog_leds(random.getrandbits(3))
