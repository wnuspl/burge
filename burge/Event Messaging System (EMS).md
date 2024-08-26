Controls interactions between [[Element]], [[Scene Manager]], and [[Component]].

---

#### [[Event Sender]]
Sends events to all receivers

#### [[Receiver]]
Holds all received events until `poll` is called

#### [[Router]]
Similar to sender, but registers receivers with a [[Uuid]] and will only send events to specific receivers
- Useful when a large amount of receivers are involved
- Receiver cannot tell if it is receiving a routed event or a regular event