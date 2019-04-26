(function($, window) {
    window.jumpVid = function (id, time) {
     $("video").each(function(e) {
      if(btoa(this.currentSrc) == id) {
       this.currentTime = time;
       this.play();
      }
     });
    };
    $(window).on('load', function() {
        $("video").each(function(e) {
            var src = this.currentSrc;
            var id = btoa(src);
            console.log(src);
            var start = window.localStorage[id];
            if (start) {
                if(!$(this).prev().find(".resume").length){
                 $(this).prev().append(' <div class="resume"></div>');
                }
                $(this).prev().find(".resume").html('Resume <a href="javascript:jumpVid(\''+id+'\',' + start + ');">@'+Math.round(start / 60) +'min');
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
