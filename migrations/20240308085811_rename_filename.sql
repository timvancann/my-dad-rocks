UPDATE songs as old SET
    audio_file_path = new.audio_file_path
FROM (VALUES
    ('Back to the Future','johnny b goode.mp3'),
    ('Black Sabbath','paranoid.mp3'),
    ('Cars','life is a highway.mp3'),
    ('Deep Blue Something','breakfast at tiffanies.mp3'),
    ('Doe Maar','smoorverliefd.mp3'),
    ('Doobie Brothers','long train running.mp3'),
    ('Foo Fighters','learn to fly.mp3'),
    ('Golden Earring','radar love.mp3'),
    ('Lenny Kravitz','always on the run.mp3'),
    ('Live','i alone.mp3'),
    ('Neil Young','rockin’ in the free world.mp3'),
    ('Paulo Mendonca','just in case.mp3'),
    ('Pearl Jam','alive.mp3'),
    ('Ram Jam','black betty.mp3'),
    ('Red Hot Chilli Peppers','californication.mp3'),
    ('Steelers Wheel','stuck in the middle.mp3'),
    ('The Free','all right now.mp3'),
    ('Three Doors Down','kryptonite.mp3'),
    ('Queens of the Stone Age','no one knows.mp3'),
    ('Van Halen', 'ain''t talkin'' ''bout love.mp3'),
    ('Imagine Dragons', 'sharks.mp3'),
    ('Living Color', 'love rears its ugly head.mp3'),
    ('Muse', 'time is running out.mp3')
) AS new(artist, audio_file_path) 
WHERE new.artist = old.artist
