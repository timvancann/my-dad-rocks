UPDATE songs as old
SET audio_file_path = new.audio_file_path
FROM (VALUES ('Back to the Future', 'johnny_b_goode'),
             ('Black Sabbath', 'paranoid'),
             ('Cars', 'life_is_a_highway'),
             ('Deep Blue Something', 'breakfast_at_tiffanies'),
             ('Doe Maar', 'smoorverliefd'),
             ('Doobie Brothers', 'long_train_running'),
             ('Foo Fighters', 'learn_to_fly'),
             ('Golden Earring', 'radar_love'),
             ('Lenny Kravitz', 'always_on_the_run'),
             ('Live', 'i_alone'),
             ('Neil Young', 'rockin_in_the_free_world'),
             ('Paulo Mendonca', 'just_in_case'),
             ('Pearl Jam', 'alive'),
             ('Ram Jam', 'black_betty'),
             ('Red Hot Chilli Peppers', 'californication'),
             ('Steelers Wheel', 'stuck_in_the_middle'),
             ('The Free', 'all_right_now'),
             ('Three Doors Down', 'kryptonite'),
             ('Queens of the Stone Age', 'no_one_knows'),
             ('Van Halen', 'aint_talkin_bout_love'),
             ('Imagine Dragons', 'sharks'),
             ('Living Color', 'love_rears_its_ugly_head'),
             ('Muse', 'time_is_running_out')) AS new(artist, audio_file_path)
WHERE new.artist = old.artist;

ALTER TABLE songs
    RENAME COLUMN audio_file_path TO sanitized_title;
