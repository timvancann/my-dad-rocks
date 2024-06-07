ALTER TABLE songs DROP COLUMN sanitized_title;
ALTER TABLE songs ADD COLUMN release_mid VARCHAR;
ALTER TABLE songs ADD COLUMN artist_mid VARCHAR;

UPDATE songs as old
SET release_mid = new.release_mid,
    artist_mid = new.artist_mid
FROM (VALUES ('Back to the Future', 'b9c54760-423f-3065-802e-a14313c87934', ''),
             ('Black Sabbath', 'd820f080-845a-4525-8e46-087ce9f8cdda', '5182c1d9-c7d2-4dad-afa0-ccfeada921a8'),
             ('Cars', 'a0582f81-c181-4b1f-bbc6-9a31ceb6a25f', '6e0ae159-8449-4262-bba5-18ec87fa529f'),
             ('Deep Blue Something', 'e01bc4d7-6482-4f48-98fc-ae36fc8e9ff2', 'ae3f6a8a-c465-4707-8667-8ce0172bc417'),
             ('Doe Maar', '80e9733e-51fa-434b-8131-9f1710aae2d1', '7a60b3f1-01ee-4656-bf4e-9891479ee101'),
             ('Doobie Brothers', '3dc6075a-ee1b-4d3d-bc4b-f2a8ab66c806', '588dea29-eea3-456b-a815-3ee04f75c8e7'),
             ('Foo Fighters', '73fa4781-d526-32e6-b23e-c8d5dc672429', '67f66c07-6e61-4026-ade5-7e782fad3a5d'),
             ('Golden Earring', 'd2a04001-9473-41a8-a4a0-4f4fe8f32aa4', '75336c3d-2833-46cb-8037-b835cd7d646d'),
             ('Lenny Kravitz', 'ac74b431-4089-4626-8748-bc3258813790', '0ef3f425-9bd2-4216-9dd2-219d2fe90f1f'),
             ('Live', '5cf8bec1-6677-405a-9d75-ac37efec5d91', 'cba77ba2-862d-4cee-a8f6-d3f9daf7211c'),
             ('Neil Young', '438612a6-e370-4622-a84b-2c9be5770cff', '75167b8b-44e4-407b-9d35-effe87b223cf'),
             ('Paulo Mendonca', '8a98a665-8359-4deb-8f99-2391fedddcb3', '076b4569-5eb9-4318-9057-658b55464346'),
             ('Pearl Jam', '8d0bc6d4-8700-44e8-90c8-b86c23e7ff14', '83b9cbe7-9857-49e2-ab8e-b57b01038103'),
             ('Ram Jam', '90323744-6881-47fe-9795-4318af253f14', 'f2be3d59-c508-492b-a553-056fc8a7e7b0'),
             ('Red Hot Chilli Peppers', 'ae9e09df-5029-30ec-bf1c-8d4a905f8c02', '8bfac288-ccc5-448d-9573-c33ea2aa5c30'),
             ('Steelers Wheel', 'cfbc501c-6f41-4618-8bb1-90bb8ebd936f', '63200203-e2c6-4081-8e0d-00c9732c6b6f'),
             ('The Free', '04eae07a-154d-358d-890e-2bf256c1343a', '6cb5d1ca-03ce-4656-92f5-bf35f53d1582'),
             ('Three Doors Down', '7aa1ae13-a22c-4749-a72e-5dceecca101e', '2386cd66-e923-4e8e-bf14-2eebe2e9b973'),
             ('Queens of the Stone Age', 'ab9e6f50-b248-4ed2-a591-1f175e609e44', '7dc8f5bd-9d0b-4087-9f73-dc164950bbd8'),
             ('Van Halen', '0d5f0dc2-b597-4b6c-9a6f-49b70b8e23b6', 'b665b768-0d83-4363-950c-31ed39317c15'),
             ('Imagine Dragons', '400bad2a-86b7-4aa3-9837-6d992f431888', '012151a8-0f9a-44c9-997f-ebd68b5389f9'),
             ('Living Color', 'ae22ec48-abbb-4adb-8c6c-4cfc296ca3cb', 'dc6f8c1f-626b-42b0-9115-7e66ae4cecd6'),
             ('Muse', 'a0a2b395-7989-4ec7-99f9-9bc9425c53b7', '9c9f1380-2516-4fc9-a3e6-f9f61941d090')) AS new(artist, release_mid, artist_mid)
WHERE new.artist = old.artist;
