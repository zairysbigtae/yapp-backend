--
-- PostgreSQL database dump
--

\restrict k8ichCx5VTUL9vVhPhlFI1LEwJoA0cYuv0qmRauugKAg7o4UYlvfDAXycn517qy

-- Dumped from database version 17.6
-- Dumped by pg_dump version 17.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: users; Type: TABLE; Schema: public; Owner: zai
--

CREATE TABLE public.users (
    id integer NOT NULL,
    name character varying(100) NOT NULL,
    password character varying(100) NOT NULL
);


ALTER TABLE public.users OWNER TO zai;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: zai
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO zai;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: zai
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: zai
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: zai
--

COPY public.users (id, name, password) FROM stdin;
1	John Doe	0cbc6611f5540bd0809a388dc95a615b
2	John Doe	0cbc6611f5540bd0809a388dc95a615b
3	John Doe	0cbc6611f5540bd0809a388dc95a615b
4	John Doe	0cbc6611f5540bd0809a388dc95a615b
\.


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: zai
--

SELECT pg_catalog.setval('public.users_id_seq', 4, true);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: zai
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

\unrestrict k8ichCx5VTUL9vVhPhlFI1LEwJoA0cYuv0qmRauugKAg7o4UYlvfDAXycn517qy

