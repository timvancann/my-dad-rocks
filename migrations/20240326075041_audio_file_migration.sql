UPDATE songs as old SET
    gs_url = new.gs_url
FROM (VALUES
    ('Back to the Future','https://storage.googleapis.com/my-dad-rocks/assets/audio/backtothefuture_johnnybgoode.mp3'),
    ('Black Sabbath','https://storage.googleapis.com/my-dad-rocks/assets/audio/blacksabbath_paranoid.mp3'),
    ('Cars','https://storage.googleapis.com/my-dad-rocks/assets/audio/cars_lifeisahighway.mp3'),
    ('Deep Blue Something','https://storage.googleapis.com/my-dad-rocks/assets/audio/deepbluesomething_breakfastattiffanies.mp3'),
    ('Doe Maar','https://storage.googleapis.com/my-dad-rocks/assets/audio/doemaar_smoorverliefd.mp3'),
    ('Doobie Brothers','https://storage.googleapis.com/my-dad-rocks/assets/audio/doobiebrothers_longtrainrunning.mp3'),
    ('Foo Fighters','https://storage.googleapis.com/my-dad-rocks/assets/audio/foofighters_learntofly.mp3'),
    ('Golden Earring','https://storage.googleapis.com/my-dad-rocks/assets/audio/goldenearring_radarlove.mp3'),
    ('Lenny Kravitz','https://storage.googleapis.com/my-dad-rocks/assets/audio/lennykravitz_alwaysontherun.mp3'),
    ('Live','https://storage.googleapis.com/my-dad-rocks/assets/audio/live_ialone.mp3'),
    ('Neil Young','https://storage.googleapis.com/my-dad-rocks/assets/audio/neilyoung_rockininthefreeworld.mp3'),
    ('Paulo Mendonca','https://storage.googleapis.com/my-dad-rocks/assets/audio/paulomendonca_justincase.mp3'),
    ('Pearl Jam','https://storage.googleapis.com/my-dad-rocks/assets/audio/pearljam_alive.mp3'),
    ('Ram Jam','https://storage.googleapis.com/my-dad-rocks/assets/audio/ramjam_blackbetty.mp3'),
    ('Red Hot Chilli Peppers','https://storage.googleapis.com/my-dad-rocks/assets/audio/redhotchillipeppers_californication.mp3'),
    ('Steelers Wheel','https://storage.googleapis.com/my-dad-rocks/assets/audio/stealerswheel_stuckinthemiddle.mp3'),
    ('The Free','https://storage.googleapis.com/my-dad-rocks/assets/audio/thefree_allrightnow.mp3'),
    ('Three Doors Down','https://storage.googleapis.com/my-dad-rocks/assets/audio/threedoorsdown_kryptonite.mp3'),
    ('Queens of the Stone Age','https://storage.googleapis.com/my-dad-rocks/assets/audio/queensofthestoneage_nooneknows.mp3'),
    ('Van Halen', 'https://storage.googleapis.com/my-dad-rocks/assets/audio/vanhalen_ainttalkinboutlove.mp3'),
    ('Living Color', 'https://storage.googleapis.com/my-dad-rocks/assets/audio/livingcolour_loverearsitsuglyhead.mp3'),
    ('Muse', 'https://storage.googleapis.com/my-dad-rocks/assets/audio/muse_timeisrunningout.mp3')
) AS new(artist, gs_url) 
WHERE new.artist = old.artist
