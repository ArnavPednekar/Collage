create table location(
location_id number primary key,
regional_group varchar(50)
);
 
create table job (
    job_id number(10) primary key,
    "function" varchar(50)
);
create table employee(
emp_id number(10) primary key,
lastname varchar(50),
firstname varchar(50),
job_id number(10),
hire_date date,
salary number(10),
dept_id number(10)); 

create table Department(
dept_id number(10) primary key,
name varchar(30),
loc_id number(10),
constraint fk_department_location
foreign key(loc_id)
references location(location_id));

insert into location values (231, 'Mumbai');
insert into location values (311, 'Chandigarh');
insert into location values (131, 'Kolkota');
insert into location values (241, 'Delhi');
insert into location values (421, 'Banglore');

insert into department values (1, 'Sales', 311);
insert into department values (2, 'R&D', 241);
insert into department values (3, 'Design', 311);
insert into department values (4, 'HR', 231);

insert into job values (11, 'Marketing specialist');
insert into job values (15, 'Content Analyst');
insert into job values (32, 'Head');
insert into job values (22, 'Support Engineer');
insert into job values (13, 'Software Engineer');

insert into employee values (1122, 'Gautam', 'Smita', 11, '2018-01-25', 45000, 1);
insert into employee values (1123, 'Naik', 'Sharvani', 15, '2016-04-20', 23000, 3);
insert into employee values (1124, 'Mehta', 'Arjun', 22, '2015-05-01', 20000, 2);
insert into employee values (1125, 'Mehta', 'Shivang', 32, '2016-10-03', 10000, 1);
insert into employee values (1126, 'Naik', 'Shiva', 15, '2015-08-04', 12000, 3);
insert into employee values (1127, 'palekar', 'Amrish', 32, '2016-03-05', 11000, 4);
insert into employee values (1128, 'Tople', 'Aarya', 13, '2018-09-03', 9000, 2);
