// Hack to resume AudioContexts on first user interaction.
// https://developers.google.com/web/updates/2018/11/web-audio-autoplay#moving-forward
// Uses capture phase so events fire before Bevy's canvas handlers can stop propagation.
// Includes touchstart for iOS Safari reliability.
(function () {
    const audioContextList = [];

    const userInputEventNames = [
        "click",
        "contextmenu",
        "auxclick",
        "dblclick",
        "mousedown",
        "mouseup",
        "pointerdown",
        "pointerup",
        "touchstart",
        "touchend",
        "keydown",
        "keyup",
    ];

    function proxyContext(original) {
        if (!original) return;
        return new Proxy(original, {
            construct(target, args) {
                const result = new target(...args);
                audioContextList.push(result);
                return result;
            },
        });
    }

    self.AudioContext = proxyContext(self.AudioContext);
    if (self.webkitAudioContext) {
        self.webkitAudioContext = proxyContext(self.webkitAudioContext);
    }

    function resumeAllContexts(_event) {
        let count = 0;

        audioContextList.forEach((context) => {
            if (context.state !== "running") {
                context.resume();
            } else {
                count++;
            }
        });

        if (count > 0 && count === audioContextList.length) {
            userInputEventNames.forEach((eventName) => {
                document.removeEventListener(eventName, resumeAllContexts, true);
            });
        }
    }

    userInputEventNames.forEach((eventName) => {
        document.addEventListener(eventName, resumeAllContexts, true);
    });
})();
