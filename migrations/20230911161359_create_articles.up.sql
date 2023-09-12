create type topic as enum ('inrikes', 'utrikes', 'lokalt', 'ekonomi', 'kultur', 'vetenskap', 'sport', 'opinion');

create table articles (
  id uuid,
  title varchar not null,
  link varchar not null,
  published timestamptz not null,
  topic topic not null,
  primary key (id)
);

create index id_idx on articles (id);
create index id_published on articles (published desc);