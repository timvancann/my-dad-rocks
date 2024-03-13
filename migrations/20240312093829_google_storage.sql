ALTER TABLE songs ADD COLUMN gs_url VARCHAR;

UPDATE songs as old SET
    gs_url = new.gs_url
FROM (VALUES
    ('Back to the Future','https://storage.googleapis.com/my-dad-rocks/website_mp3/johnny%20b%20goode.mp3'),
    ('Black Sabbath','https://storage.googleapis.com/my-dad-rocks/website_mp3/paranoid.mp3'),
    ('Cars','https://storage.googleapis.com/my-dad-rocks/website_mp3/life%20Is%20a%20highway.mp3'),
    ('Deep Blue Something','https://storage.googleapis.com/my-dad-rocks/website_mp3/breakfast%20at%20tiffanies.mp3'),
    ('Doe Maar','https://storage.googleapis.com/my-dad-rocks/website_mp3/smoorverliefd.mp3'),
    ('Doobie Brothers','https://storage.googleapis.com/my-dad-rocks/website_mp3/long%20train%20running.mp3'),
    ('Foo Fighters','https://storage.googleapis.com/my-dad-rocks/website_mp3/learn%20to%20fly.mp3'),
    ('Golden Earring','https://storage.googleapis.com/my-dad-rocks/website_mp3/radar%20love.mp3'),
    ('Lenny Kravitz','https://storage.googleapis.com/my-dad-rocks/website_mp3/always%20on%20the%20run.mp3'),
    ('Live','https://storage.googleapis.com/my-dad-rocks/website_mp3/i%20alone.mp3'),
    ('Neil Young','https://storage.googleapis.com/my-dad-rocks/website_mp3/rockin%E2%80%99%20in%20the%20free%20world.mp3'),
    ('Paulo Mendonca','https://storage.googleapis.com/my-dad-rocks/website_mp3/just%20in%20case.mp3'),
    ('Pearl Jam','https://storage.googleapis.com/my-dad-rocks/website_mp3/alive.mp3'),
    ('Ram Jam','https://storage.googleapis.com/my-dad-rocks/website_mp3/black%20betty.mp3'),
    ('Red Hot Chilli Peppers','https://storage.googleapis.com/my-dad-rocks/website_mp3/californication.mp3'),
    ('Steelers Wheel','https://storage.googleapis.com/my-dad-rocks/website_mp3/stuck%20in%20the%20middle.mp3'),
    ('The Free','https://storage.googleapis.com/my-dad-rocks/website_mp3/all%20right%20now.mp3'),
    ('Three Doors Down','https://storage.googleapis.com/my-dad-rocks/website_mp3/kryptonite.mp3'),
    ('Queens of the Stone Age','https://storage.googleapis.com/my-dad-rocks/website_mp3/no%20one%20know.mp3'),
    ('Van Halen', 'https://storage.googleapis.com/my-dad-rocks/website_mp3/aint%20talkin%20bout%20love.mp3'),
    ('Living Color', 'https://storage.googleapis.com/my-dad-rocks/website_mp3/love%20rears%20its%20ugly%20head.mp3'),
    ('Muse', 'https://storage.googleapis.com/my-dad-rocks/website_mp3/time%20is%20running%20out.mp3')
) AS new(artist, gs_url) 
WHERE new.artist = old.artist
