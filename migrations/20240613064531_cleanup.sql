DELETE FROM songs WHERE artist = 'Cars';
DELETE FROM songs WHERE artist = 'Guns ''n Roses';
DELETE FROM songs WHERE artist = 'Alice Cooper';
DELETE FROM songs WHERE artist = 'Lenny Kravitz';

INSERT INTO songs (artist, title, release_mid, artist_mid, gs_url)
VALUES
    ('Alice Cooper','Schools''s Out','852818f9-5614-3c6c-9ab7-df65bb9ca143', '4d7928cd-7ed2-4282-8c29-c0c9f966f1bd', 'https://storage.googleapis.com/my-dad-rocks/assets/audio/alicecooper_schoolsout.mp3'),
    ('Alien Ant Farm','Smooth Criminal','9af30d61-292b-3c1d-bd4b-ff02da591017', '8ac6cc32-8ddf-43b1-9ac4-4b04f9053176', 'https://storage.googleapis.com/my-dad-rocks/assets/audio/alienantfarm_smoothcriminal.mp3');
