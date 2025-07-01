CREATE TABLE IF NOT EXISTS employee (
    id VARCHAR(256) PRIMARY KEY,
    first_name VARCHAR(256) NOT NULL,
    middle_name VARCHAR(256),
    last_name VARCHAR(256) NOT NULL,
    dob DATE NOT NULL,
    nationality TEXT NOT NULL,
    degree VARCHAR(256) NOT NULL,
    gender VARCHAR(10) NOT NULL,
    marital_status VARCHAR(16)
        CHECK (marital_status IN ('single','married','divorced','widowed')),
    address TEXT NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS employee_phonenumber (
    employee_id VARCHAR(256) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (employee_id, phone_number),
    FOREIGN KEY (employee_id) REFERENCES employee(id)
);

CREATE TABLE IF NOT EXISTS employee_details (
    employee_id VARCHAR(256) PRIMARY KEY,
    position VARCHAR(256) NOT NULL,
    salary NUMERIC(10,2) NOT NULL,
    work_location VARCHAR(256) CHECK (work_location IN ('remote', 'on-site', 'hybrid')),
    start_date DATE NOT NULL,
    end_date DATE,
    FOREIGN KEY (employee_id) REFERENCES employee(id)
);

CREATE TABLE IF NOT EXISTS employee_attendance (
    employee_id VARCHAR(256) NOT NULL,
    log_date DATE NOT NULL,
    PRIMARY KEY (employee_id, log_date),
    FOREIGN KEY (employee_id) REFERENCES employee(id)
);

CREATE TABLE IF NOT EXISTS department (
    id VARCHAR(256) PRIMARY KEY,
    d_name VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    supervisor_id VARCHAR(256) NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (d_name, supervisor_id),
    FOREIGN KEY (supervisor_id) REFERENCES employee(id)
);

CREATE TABLE IF NOT EXISTS teacher (
    id VARCHAR(256) PRIMARY KEY,
    employee_id VARCHAR(256) NOT NULL,
    department_id VARCHAR(256) NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employee(id),
    FOREIGN KEY (department_id) REFERENCES department(id)
);

CREATE TABLE IF NOT EXISTS tutorial (
    id VARCHAR(256) PRIMARY KEY,
    code VARCHAR(256) NOT NULL,
    grade INT NOT NULL,
    max_capacity INT NOT NULL,
    OVERALL_SCORE INT NOT NULL
);

CREATE TABLE IF NOT EXISTS student (
    id VARCHAR(256) PRIMARY KEY,
    first_name VARCHAR(256) NOT NULL,
    middle_name VARCHAR(256) NOT NULL,
    last_name VARCHAR(256) NOT NULL,
    dob DATE NOT NULL,
    nationality TEXT NOT NULL,
    grade INT NOT NULL,
    tutorial_id VARCHAR(256),
    FOREIGN KEY (tutorial_id) REFERENCES tutorial(id)
);

CREATE TABLE IF NOT EXISTS student_email (
    username VARCHAR(256) PRIMARY KEY,
    email VARCHAR(256) NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    student_password VARCHAR(256) NOT NULL,
    student_id VARCHAR(256) NOT NULL,
    UNIQUE (email),
    FOREIGN KEY (student_id) REFERENCES student(id)
);

CREATE TABLE IF NOT EXISTS student_attendance (
    student_id VARCHAR(256) NOT NULL,
    log_date DATE NOT NULL,
    PRIMARY KEY (student_id, log_date),
    FOREIGN KEY (student_id) REFERENCES student(id)
);

CREATE TABLE IF NOT EXISTS monthly_report (
    student_id VARCHAR(256) NOT NULL,
    teacher_id VARCHAR(256) NOT NULL,
    log_date DATE NOT NULL,
    class_participation NUMERIC NOT NULL,
    social_participation NUMERIC(5,2) NOT NULL,
    projects_grades NUMERIC(5,2) NOT NULL,
    individual_projects INT NOT NULL,
    PRIMARY KEY (student_id, log_date, teacher_id),
    FOREIGN KEY (teacher_id) REFERENCES teacher(id),
    FOREIGN KEY (student_id) REFERENCES student(id)
);

CREATE TABLE IF NOT EXISTS student_overall_status (
  student_id VARCHAR(256) PRIMARY KEY,
  overall_class_participation NUMERIC NOT NULL,
  overall_social_participation NUMERIC(5,2) NOT NULL,
  overall_projects_grades NUMERIC(5,2) NOT NULL,
  overall_individual_projects INT NOT NULL,
  spoken_languages JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS course (
    id VARCHAR(256) PRIMARY KEY,
    c_name VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    department_id VARCHAR(256) NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (department_id) REFERENCES department(id),
    UNIQUE (c_name)
);

CREATE TABLE IF NOT EXISTS work_assignment (
    id VARCHAR(256) PRIMARY KEY,
    course_id VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    grade INT NOT NULL,
    supervisor_id VARCHAR(256) NOT NULL,
    FOREIGN KEY (supervisor_id) REFERENCES teacher(id)
);

CREATE TABLE IF NOT EXISTS student_assignment (
    student_id VARCHAR(256) NOT NULL,
    assignment_id VARCHAR(256) NOT NULL,
    grade INT NOT NULL,
    duedate DATE NOT NULL,
    teacher_id VARCHAR(25) NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (student_id, assignment_id),
    FOREIGN KEY (student_id) REFERENCES student(id),
    FOREIGN KEY (teacher_id) REFERENCES teacher(id)
    FOREIGN KEY (assignment_id) REFERENCES work_assignment(id)
);

CREATE TABLE IF NOT EXISTS grade_student (
    student_id VARCHAR(256) NOT NULL,
    assignment_id VARCHAR(256) NOT NULL,
    teacher_id VARCHAR(256) NOT NULL,
    student_grade INT NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (student_id, assignment_id, teacher_id),
    FOREIGN KEY (student_id) REFERENCES student(id),
    FOREIGN KEY (assignment_id) REFERENCES work_assignment(id),
    FOREIGN KEY (teacher_id) REFERENCES teacher(id)
);

CREATE TABLE IF NOT EXISTS employee_email (
    employee_id VARCHAR(256) NOT NULL,
    email VARCHAR(256) NOT NULL,
    password VARCHAR(256) NOT NULL,
    constructed_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    mail_code VARCHAR(256) NOT NULL,
    UNIQUE (email),
    PRIMARY KEY (employee_id, email),
    FOREIGN KEY (employee_id) REFERENCES employee(id)
);

CREATE TABLE IF NOT EXISTS intern (
    id VARCHAR(256) PRIMARY KEY,
    employee_id VARCHAR(256) NOT NULL, -- Added this line
    first_name VARCHAR(256) NOT NULL,
    middle_name VARCHAR(256),
    last_name VARCHAR(256) NOT NULL,
    dob DATE NOT NULL,
    nationality TEXT NOT NULL,
    school VARCHAR(256) NOT NULL,
    work_role VARCHAR(256) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    FOREIGN KEY (employee_id) REFERENCES employee(id),
    UNIQUE (employee_id, start_date, end_date)
);

CREATE TABLE IF NOT EXISTS course_tutorial(
    course_id VARCHAR(256) NOT NULL,
    tutorial_id VARCHAR(256) NOT NULL,
    PRIMARY KEY (course_id, tutorial_id),
    FOREIGN KEY (course_id) REFERENCES course(id),
    FOREIGN KEY (tutorial_id) REFERENCES tutorial(id)
);

CREATE TABLE IF NOT EXISTS tutorial_teacher (
    tutorial_id VARCHAR(256) NOT NULL,
    teacher_id VARCHAR(256) NOT NULL,
    PRIMARY KEY (tutorial_id, teacher_id),
    FOREIGN KEY (tutorial_id) REFERENCES tutorial(id),
    FOREIGN KEY (teacher_id) REFERENCES teacher(id)
);

CREATE OR REPLACE VIEW teacher_view AS
SELECT
    e.first_name AS first_name,
    e.middle_name AS middle_name,
    e.last_name AS last_name,
    d.d_name
FROM
    employee e
JOIN
    teacher t ON e.id = t.employee_id
JOIN
    department d ON t.department_id = d.id;

CREATE OR REPLACE VIEW teacher_view_details AS
SELECT
    e.id AS id
    e.first_name AS first_name,
    e.middle_name AS middle_name,
    e.last_name AS last_name,
    d.d_name
FROM
    employee e
JOIN
    teacher t ON id = t.employee_id
JOIN
    department d ON t.department_id = d.id;



CREATE OR REPLACE VIEW supervisor_details_helper AS
SELECT t.id,
    e.first_name,
    e.middle_name,
    e.last_name,
    d.d_name
   FROM employee e,
    department d,
    teacher t
  WHERE t.employee_id= e.id
  AND t.department_id = d.id 
  AND e.id: = d.supervisor_id;


CREATE OR REPLACE FUNCTION tutorial_students(
    tutorial_id VARCHAR(256)
)
RETURNS TABLE (
    first_name VARCHAR(256),
    middle_name VARCHAR(256),
    last_name VARCHAR(256),
    code VARCHAR(256)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.first_name,
        s.middle_name,
        s.last_name,
        t.code
    FROM
        student s
    JOIN
        tutorial t ON s.tutorial_id = t.id
    WHERE
        t.id = tutorial_id;
END;
$$;

CREATE OR REPLACE FUNCTION teacher_details(
    department_name VARCHAR(256)
)
RETURNS TABLE (
    first_name VARCHAR(256),
    middle_name VARCHAR(256),
    last_name VARCHAR(256),
    d_name VARCHAR(256),
    description TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        e.first_name,
        e.middle_name,
        e.last_name,
        d.d_name,
        d.description
    FROM
        employee e
    JOIN
        teacher t ON e.id = t.employee_id
    JOIN
        department d ON t.department_id = d.id
    WHERE
        d.d_name = department_name;
END;
$$;

CREATE OR REPLACE FUNCTION classroom_detail(
    t_code VARCHAR(256)
)
RETURNS TABLE (
    first_name VARCHAR(256),
    middle_name VARCHAR(256),
    last_name VARCHAR(256),
    code VARCHAR(256)
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.first_name,
        s.middle_name,
        s.last_name,
        t.code
    FROM
        student s
    JOIN
        tutorial t ON s.tutorial_id = t.id
    WHERE
        t.code = t_code;
END;
$$;

CREATE OR REPLACE FUNCTION student_details(
    email VARCHAR(256)
)
RETURNS TABLE (
    first_name VARCHAR(256),
    middle_name VARCHAR(256),
    last_name VARCHAR(256),
    nationality TEXT,
    grade INT,
    tutorial VARCHAR(256),
    stu_email VARCHAR(256)  -- renamed from "email"
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.first_name,
        s.middle_name,
        s.last_name,
        s.nationality,
        s.grade,
        t.code AS tutorial,
        se.email AS stu_email  -- alias to match output column name
    FROM
        student s
    JOIN
        student_email se ON s.id = se.student_id
    JOIN
        tutorial t ON s.tutorial_id = t.id
    WHERE
        se.email = email;  
END;
$$;

create or replace function student_attendance_record(
    email VARCHAR(256)
)
RETURNS TABLE (
    first_name VARCHAR(256),
    middle_name VARCHAR(256),
    last_name VARCHAR(256),
    tutorial_code VARCHAR(256),
    attendance_count INT,
    log_date DATE
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.first_name,
        s.middle_name,
        s.last_name,
        t.code AS tutorial_code,
        COUNT(sa.log_date) AS attendance_count,
        sa.log_date
    FROM
        student s
    JOIN
        student_email se ON s.id = se.student_id
    JOIN
        tutorial t ON s.tutorial_id = t.id
    JOIN
        student_attendance sa ON s.id = sa.student_id
    WHERE
        se.email = email
    GROUP BY
        s.first_name, s.middle_name, s.last_name, t.code, sa.log_date;
END;
$$;

CREATE OR REPLACE FUNCTION fetch_tutorial_student_assigment(
    tutorial_id VARCHAR(256),
    assignment_id VARCHAR(256)
)
RETURNS TABLE (
    id VARCHAR(256),
    first_name VARCHAR(256),
    middle_name VARCHAR(256),
    last_name VARCHAR(256),
    code VARCHAR(256),
    assignment_grade INT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.id,
        s.first_name,
        s.middle_name,
        s.last_name,
        t.code,
        sa.grade AS assignment_grade
    FROM
        student s
    JOIN
        tutorial t ON s.tutorial_id = t.id
    JOIN
        student_assignment sa ON s.id = sa.student_id
    WHERE
        t.id = tutorial_id AND sa.assignment_id = assignment_id;
END;
$$;

CREATE OR REPLACE PROCEDURE grade_students_bulk(
    IN grades_data JSONB
)
LANGUAGE plpgsql
AS $$
DECLARE
    item JSONB;
BEGIN
    FOR item IN SELECT * FROM jsonb_array_elements(grades_data)
    LOOP
        INSERT INTO grade_student (
            student_id,
            assignment_id,
            teacher_id,
            student_grade
        )
        VALUES (
            item->>'student_id',
            item->>'assignment_id',
            item->>'teacher_id',
            (item->>'student_grade')::INT
        )
        ON CONFLICT (student_id, assignment_id, teacher_id) DO UPDATE
        SET student_grade = EXCLUDED.student_grade,
            constructed_date = CURRENT_TIMESTAMP;
    END LOOP;
END;
$$;


