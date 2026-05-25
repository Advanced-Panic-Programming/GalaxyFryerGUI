Hybrid communication system:
    1) Orchestrator to GUi communication process:
        - Orchestrator sends message on channel
        - Communication module receives it, reads it and generates the corresponding event using Bevy Messages
        - The correct GUI module will listen for that particular event and react to it
        
    2) Gui to Orchestrator communication process:
        Since the communication is simple and doesn't impact the GUI, I decided to opt for a simpler approach. 
        The GUI element that would cause the event that we need to notify to the orchestrator will be responsible
        for sending it in the channel right away, without going through the communication module.
        
!!! Note: the event messages definitions are inside galaxy_view/messages !!!