create extension if not exists "uuid-ossp";

create or replace function set_updated_at()
returns trigger as 
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;