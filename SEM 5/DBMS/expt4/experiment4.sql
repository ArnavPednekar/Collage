-- ====================================================================
-- Experiment No: 3 & 4: SQL Queries, Aggregate Functions & Bank Database
-- ====================================================================

-- ====================================================================
-- PART 1: EMPLOYEE DATABASE & QUERIES
-- ====================================================================

CREATE DATABASE IF NOT EXISTS employee_db;
USE employee_db;

DROP TABLE IF EXISTS employee;

CREATE TABLE employee (
    empid INT PRIMARY KEY,
    lname VARCHAR(50),
    fname VARCHAR(50),
    jobid INT,
    hire_date DATE,
    salary DECIMAL(10, 2),
    deptid INT
);

INSERT INTO employee (empid, lname, fname, jobid, hire_date, salary, deptid) VALUES
(1122, 'Gautam', 'Smita', 11, '2018-01-25', 45000.00, 1),
(1123, 'Naik', 'Sharvani', 15, '2016-04-20', 23000.00, 3),
(1124, 'Mehta', 'Arjun', 22, '2015-05-01', 20000.00, 2),
(1125, 'Mehta', 'Shivang', 32, '2016-10-03', 10000.00, 1),
(1126, 'Naik', 'Shiva', 15, '2015-08-04', 12000.00, 3),
(1127, 'palekar', 'Amrish', 32, '2016-03-05', 11000.00, 4),
(1128, 'Tople', 'Aarya', 13, '2018-09-03', 9000.00, 2);

-- 1. Rename empid as id_of_the employee, lname as name_of_the_employee, deptid as dept_id from employee
SELECT empid AS id_of_the_employee, lname AS name_of_the_employee, deptid AS dept_id 
FROM employee;

-- 2. Display the distinct lnames of employee
SELECT DISTINCT lname 
FROM employee;

-- 3. Display the lname and annual salary of employee
SELECT lname, salary * 12 AS annual_salary 
FROM employee;

-- 4. Display all employee whose salary is in range 20000 and 40000
SELECT * 
FROM employee 
WHERE salary BETWEEN 20000 AND 40000;

-- 5. Display the details of employees except working for department 10 or 30
SELECT * 
FROM employee 
WHERE deptid NOT IN (10, 30);

-- 6. Display the details of employee whose firstname starts with j
SELECT * 
FROM employee 
WHERE fname LIKE 'j%';

-- 7. Display the details of employee whose firstname starts with j and ends with n
SELECT * 
FROM employee 
WHERE fname LIKE 'j%n';

-- 8. Display the details of employee whose firstname starts with j or k or c
SELECT * 
FROM employee 
WHERE fname LIKE 'j%' OR fname LIKE 'k%' OR fname LIKE 'c%';

-- 9. Display details of employee working for deptid=3 and salary greater than 20000
SELECT * 
FROM employee 
WHERE deptid = 3 AND salary > 20000;

-- 10. Display the empid and lastname in ascending order
SELECT empid, lname 
FROM employee 
ORDER BY empid ASC, lname ASC;

-- 11. Display the empid and lastname in descending order
SELECT empid, lname 
FROM employee 
ORDER BY empid DESC, lname DESC;

-- 12. Display the first 3 rows of employee table
SELECT * 
FROM employee 
LIMIT 3;

-- 13. Display the first name in uppercase
SELECT UPPER(fname) AS fname_upper 
FROM employee;

-- 14. Display the first name in lowercase
SELECT LOWER(fname) AS fname_lower 
FROM employee;

-- 15. Display the first name where first alphabet is capital
SELECT CONCAT(UPPER(SUBSTRING(fname, 1, 1)), LOWER(SUBSTRING(fname, 2))) AS fname_capitalized 
FROM employee;

-- 16. Display average salary of employees
SELECT AVG(salary) AS average_salary 
FROM employee;

-- 17. Display the sum of salary of employees
SELECT SUM(salary) AS total_salary 
FROM employee;

-- 18. Display the details of employee with maximum salary
SELECT * 
FROM employee 
WHERE salary = (SELECT MAX(salary) FROM employee);

-- 19. Display the details of employee with minimum salary
SELECT * 
FROM employee 
WHERE salary = (SELECT MIN(salary) FROM employee);

-- 20. Write a query to find name of employee having jobid=32 and deptid=1
SELECT fname, lname 
FROM employee 
WHERE jobid = 32 AND deptid = 1;

-- 21. Write a query to find name of employees whose name contain ‘r’ as second character
SELECT fname, lname 
FROM employee 
WHERE fname LIKE '_r%' OR lname LIKE '_r%';

-- 22. Write a query to find name of employees whose name contain at least two ‘a’
SELECT fname, lname 
FROM employee 
WHERE (LENGTH(fname) - LENGTH(REPLACE(LOWER(fname), 'a', ''))) >= 2 
   OR (LENGTH(lname) - LENGTH(REPLACE(LOWER(lname), 'a', ''))) >= 2;

-- 23. Update some record from employee table
UPDATE employee 
SET jobid = 18 
WHERE empid = 1128;

-- 24. Delete any record from employee table
DELETE FROM employee 
WHERE empid = 1128;

-- 25. Change the salary of employee where empid=1122
UPDATE employee 
SET salary = 48000.00 
WHERE empid = 1122;


-- ====================================================================
-- PART 2: BANK DATABASE & EXERCISE QUESTIONS
-- ====================================================================

-- (ii) Tables are created under the database ‘bank’
CREATE DATABASE IF NOT EXISTS bank;
USE bank;

-- Drop existing tables in reverse order of foreign key dependencies
DROP TABLE IF EXISTS Borrower;
DROP TABLE IF EXISTS Depositor;
DROP TABLE IF EXISTS Accounts_in;
DROP TABLE IF EXISTS Loan;
DROP TABLE IF EXISTS Deposit;
DROP TABLE IF EXISTS Bank_brn;
DROP TABLE IF EXISTS Customer;

-- (i) Table creation with necessary constraints (Primary Keys, Foreign Keys)

CREATE TABLE Customer (
    cid INT PRIMARY KEY,
    cname VARCHAR(100) NOT NULL,
    loc VARCHAR(100) NOT NULL,
    sex CHAR(1) CHECK (sex IN ('M', 'F', 'Other')),
    dob DATE
);

CREATE TABLE Bank_brn (
    bcode INT PRIMARY KEY,
    bloc VARCHAR(100) NOT NULL,
    bstate VARCHAR(100) NOT NULL
);

CREATE TABLE Deposit (
    dacno INT PRIMARY KEY,
    dtype VARCHAR(50) NOT NULL,
    ddate DATE,
    damt DECIMAL(12, 2) CHECK (damt > 0)
);

CREATE TABLE Loan (
    lacno INT PRIMARY KEY,
    ltype VARCHAR(50) NOT NULL,
    ldate DATE,
    lamt DECIMAL(12, 2) CHECK (lamt > 0)
);

CREATE TABLE Accounts_in (
    bcode INT,
    cid INT,
    PRIMARY KEY (bcode, cid),
    FOREIGN KEY (bcode) REFERENCES Bank_brn(bcode) ON DELETE CASCADE,
    FOREIGN KEY (cid) REFERENCES Customer(cid) ON DELETE CASCADE
);

CREATE TABLE Depositor (
    cid INT,
    dacno INT,
    PRIMARY KEY (cid, dacno),
    FOREIGN KEY (cid) REFERENCES Customer(cid) ON DELETE CASCADE,
    FOREIGN KEY (dacno) REFERENCES Deposit(dacno) ON DELETE CASCADE
);

CREATE TABLE Borrower (
    cid INT,
    lacno INT,
    PRIMARY KEY (cid, lacno),
    FOREIGN KEY (cid) REFERENCES Customer(cid) ON DELETE CASCADE,
    FOREIGN KEY (lacno) REFERENCES Loan(lacno) ON DELETE CASCADE
);

-- (v) Insert at least 5 tuples in each table

INSERT INTO Customer (cid, cname, loc, sex, dob) VALUES
(101, 'Rahul Sharma', 'Mumbai', 'M', '1990-05-12'),
(102, 'Priya Patil', 'Mumbai', 'F', '1995-08-22'),
(103, 'Amit Roy', 'Kolkata', 'M', '1988-11-03'),
(104, 'Sneha Sen', 'Kolkata', 'F', '1992-02-15'),
(105, 'Vikram Joshi', 'Delhi', 'M', '1985-07-30');

INSERT INTO Bank_brn (bcode, bloc, bstate) VALUES
(1, 'Nariman Point', 'Maharashtra'),
(2, 'Andheri', 'Maharashtra'),
(3, 'Park Street', 'West Bengal'),
(4, 'Salt Lake', 'West Bengal'),
(5, 'Connaught Place', 'Delhi');

INSERT INTO Deposit (dacno, dtype, ddate, damt) VALUES
(5001, 'Savings', '2023-01-10', 50000.00),
(5002, 'Current', '2023-02-15', 150000.00),
(5003, 'Fixed Deposit', '2022-05-20', 250000.00),
(5004, 'Savings', '2023-03-01', 30000.00),
(5005, 'Recurring', '2023-04-12', 12000.00);

INSERT INTO Loan (lacno, ltype, ldate, lamt) VALUES
(9001, 'Home Loan', '2021-06-10', 2500000.00),
(9002, 'Car Loan', '2022-08-14', 600000.00),
(9003, 'Personal Loan', '2023-01-05', 150000.00),
(9004, 'Education Loan', '2020-07-19', 800000.00),
(9005, 'Home Loan', '2022-11-25', 3000000.00);

INSERT INTO Accounts_in (bcode, cid) VALUES
(1, 101),
(2, 102),
(3, 103),
(4, 104),
(5, 105);

INSERT INTO Depositor (cid, dacno) VALUES
(101, 5001),
(102, 5002),
(103, 5003),
(104, 5004),
(105, 5005);

INSERT INTO Borrower (cid, lacno) VALUES
(101, 9001),
(102, 9002),
(103, 9003),
(104, 9004),
(105, 9005);

-- (iii) Display all the tables in bank database
SHOW TABLES;

-- (iv) Describe the structure of all tables
DESCRIBE Customer;
DESCRIBE Bank_brn;
DESCRIBE Deposit;
DESCRIBE Loan;
DESCRIBE Accounts_in;
DESCRIBE Depositor;
DESCRIBE Borrower;

-- (vi) Display the branch details
SELECT * 
FROM Bank_brn;

-- (vii) List the customers of ‘Mumbai’ city
SELECT * 
FROM Customer 
WHERE loc = 'Mumbai';

-- (viii) List the male customers of ‘Kolkata’ city
SELECT * 
FROM Customer 
WHERE loc = 'Kolkata' AND sex = 'M';

-- (ix) List the state having more than one branch
SELECT bstate 
FROM Bank_brn 
GROUP BY bstate 
HAVING COUNT(*) > 1;

-- (x) List the deposit schemes provided by the bank to the customers
SELECT DISTINCT dtype 
FROM Deposit;

-- (xi) Delete the entire content of any table (e.g., Accounts_in)
DELETE FROM Accounts_in;
