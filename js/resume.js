(function($, window) {
    $(window).on('load', function() {
        $("video").each(function(e) {
            var src = this.currentSrc;
            console.log(src);
            var start = window.localStorage[btoa(src)];
            if (start) {
                this.currentTime = parseFloat(start);
            }
        });
    });
    setInterval(function() {
        $("video").each(function(e) {
            var src = this.currentSrc;
            var time = this.currentTime;
            if (time) {
                window.localStorage[btoa(src)] = time;
                console.log("Persisted video progress at: " + time);
            }
        });
    }, 5000);
}($, window));
