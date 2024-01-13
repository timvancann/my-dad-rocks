INSERT INTO songs (artist, title, audio_file_path)
VALUES 
    ('Back to the Future','Johnny B Goode','./mp3/johnny b goode.mp3'),
    ('Black Sabbath','Paranoid','./mp3/paranoid.mp3'),
    ('Cars','Life is a Highway','./mp3/life is a highway.mp3'),
    ('Deep Blue Something','Breakfast at Tiffany''s','./mp3/breakfast at tiffanies.mp3'),
    ('Doe Maar','Smoorverliefd','./mp3/smoorverliefd.mp3'),
    ('Doobie Brothers','Long Train Running','./mp3/long train running.mp3'),
    ('Foo Fighters','Learn to Fly','./mp3/learn to fly.mp3'),
    ('Golden Earring','Radar Love','./mp3/radar love.mp3'),
    ('Lenny Kravitz','Always on the Run','./mp3/always on the run.mp3'),
    ('Live','I Alone','./mp3/i alone.mp3'),
    ('Neil Young','Rockin'' in the Free World','./mp3/rockin’ in the free world.mp3'),
    ('Paulo Mendonca','Just in Case','./mp3/just in case.mp3'),
    ('Pearl Jam','Alive','./mp3/alive.mp3'),
    ('Ram Jam','Black Betty','./mp3/black betty.mp3'),
    ('Red Hot Chilli Peppers','Californication','./mp3/californication.mp3'),
    ('Steelers Wheel','Stuck in the Middle','./mp3/stuck in the middle.mp3'),
    ('The Free','All Right Now','./mp3/all right now.mp3'),
    ('Three Doors Down','Kryptonite','./mp3/kryptonite.mp3'),
    ('Queens of the Stone Age','No One Knows','./mp3/no one knows.mp3'),
    ('Van Halen','Ain''t Talkin'' ''Bout Love', './mp3/ain''t talkin'' ''bout love.mp3'),
    ('Imagine Dragons','Sharks', './mp3/sharks.mp3'),
    ('Living Color','Love Rears Its Ugly Head', './mp3/love rears its ugly head.mp3'),
    ('Muse','Time is Running Out', './mp3/time is running out.mp3');


  UPDATE songs SET last_played_at = '2024-01-03' WHERE title = 'Just in Case';
  UPDATE songs SET last_played_at = '2024-01-03' WHERE title = 'Learn to Fly';
  UPDATE songs SET last_played_at = '2024-01-03' WHERE title = 'Paranoid';
  UPDATE songs SET last_played_at = '2024-01-03' WHERE title = 'Smoorverliefd';

  UPDATE songs SET last_played_at = '2024-01-10' WHERE title = 'Long Train Running';
  UPDATE songs SET last_played_at = '2024-01-10' WHERE title = 'Alive';
  UPDATE songs SET last_played_at = '2024-01-10' WHERE title = 'Breakfast at Tiffany''s';
  UPDATE songs SET last_played_at = '2024-01-10' WHERE title = 'No One Knows';

