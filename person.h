/*
 * person.h
 * Copyright (C) 2019 Tim Hughes
 *
 * Distributed under terms of the MIT license.
 */

#ifndef PERSON_H
#define PERSON_H

typedef struct APerson  {
    const char * name;
    const char * long_name;
} APerson ;

APerson *get_person(const char * name, const char * long_name);
void free_person(APerson *person);

typedef void (*CallBackFuncPtr)(void);
void do_some_work(CallBackFuncPtr cb_func);

typedef void (*CallBackFuncPtr2)(void *cb_data);
void do_some_work2(CallBackFuncPtr2 cb_func, void *cb_data);

#endif /* !PERSON_H */
