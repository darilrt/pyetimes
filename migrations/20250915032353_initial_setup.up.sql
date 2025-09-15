CREATE TABLE IF NOT EXISTS public.authors (
	id serial4 NOT NULL,
	"name" text NOT NULL,
	email text NOT NULL,
	password_hash text NOT NULL,
	bio text NULL,
	profile_image text NULL,
	created_at timestamp DEFAULT now() NULL,
	can_publish bool DEFAULT false NULL,
	CONSTRAINT authors_email_key UNIQUE (email),
	CONSTRAINT authors_pkey PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS public.sections (
	id serial4 NOT NULL,
	priority int4 NOT NULL,
	title varchar(255) NOT NULL,
	CONSTRAINT sections_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.articles (
	id serial4 NOT NULL,
	title varchar(255) NOT NULL,
	"content" text NOT NULL,
	author_id int4 NOT NULL,
	published bool DEFAULT false NOT NULL,
	tags _text DEFAULT '{}'::text[] NULL,
	created_at timestamp DEFAULT CURRENT_TIMESTAMP NULL,
	updated_at timestamp DEFAULT CURRENT_TIMESTAMP NULL,
	slug varchar(255) NOT NULL,
	excerpt text DEFAULT ''::text NOT NULL,
	published_at timestamp NULL,
	section_id int4 NULL,
	CONSTRAINT articles_pkey PRIMARY KEY (id),
	CONSTRAINT articles_slug_key UNIQUE (slug),
	CONSTRAINT articles_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.authors(id),
	CONSTRAINT articles_section_id_fkey FOREIGN KEY (section_id) REFERENCES public.sections(id)
);

CREATE TABLE IF NOT EXISTS public.main_story (
	id serial4 NOT NULL,
	article_id int4 NOT NULL,
	disabled bool DEFAULT false NOT NULL,
	CONSTRAINT main_story_pkey PRIMARY KEY (id),
	CONSTRAINT fk_article FOREIGN KEY (article_id) REFERENCES public.articles(id) ON DELETE RESTRICT ON UPDATE CASCADE
);