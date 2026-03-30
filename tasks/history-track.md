* Context: egui tas SSB Inspector.
* Consider the following scenario:
  * user presses "cont"
  * user presses "stop"
  * Expected: a new record in history should be added
* Consider another scenario
    * user presses "record"
    * user presses "stop"
    * Expected: a new record in history should be added.
* Consider this scenario
    * user presses "play"
  * user presses "stop"
  * Expected: no new record is added to history, player didn't do anything.
* So I'm thinking, a new history entry should be added saying "Recorded 23:03" or "Continued 23:03, total 53:03". Note that there are 100 frames per second.
* Consider also this scenario
  * User presses record
  * User gets to finish.
  * Game crashes.
  * Expected: Ideally it would be possible to recover up to the end.