CREATE TABLE plan (
	id int not null AUTO_INCREMENT,
	day int not null,
	class varchar(255) not null,
	start_hour int not null,
	end_hour int not null,
	old_fach varchar(255) not null,
	new_fach varchar(255),
	away varchar(255),
	sub varchar(255),
	room varchar(255),
	typ varchar(255),
	info varchar(255),
	PRIMARY KEY(id),
	FOREIGN KEY(day) REFERENCES day(id)
);
