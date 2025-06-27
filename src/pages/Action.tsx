import React from 'react';
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { invoke } from '@tauri-apps/api/core';

import type { Action } from "../types";
// import SubmitActionButton from '../components/Invoke/SubmitAction';
import { simplifyPath } from '@/utils'
import { useNavigate } from 'react-router-dom';
import {
    Card,
    CardAction,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select"
import { Button } from '@/components/ui/button';

const formSchema = z.object({
    name: z.string().min(2, {
        message: "action name must be at least 2 characters.",
    }),
})

const ActionModify: React.FC = () => {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
        },
    })
    const navigate = useNavigate();
    const initialValues: Action = {
        name: "",
        desc: "",
        command: "",
        args: [],
        type: "open_file",
        wait: 1,
        retry: 0,
    };

    const actionTypeOptions = [
        { value: "open_file", label: "Open File" },
        { value: "open_dir", label: "Open Directory" },
        { value: "exec_command", label: "Execute Command" },
        { value: "open_url", label: "Open URL" },
    ];
    function onSubmit(values: z.infer<typeof formSchema>) {
        console.log(values)
    }
    function handleChange(){
        console.log(form.getValues())
    }
    return (
        <Card className="action-modify" >
            <CardContent>
                <Form {...form} >
                    <form 
                    onChange={handleChange}
                    onSubmit={form.handleSubmit(onSubmit)} 
                    className="grid gap-4 grid-cols-2 grid-flow-row-dense">
                            <FormField
                                control={form.control}
                                name="name"
                                render={({ field }) => (
                                    <FormItem className='w-5/6'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Action Name</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter action name"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="type"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Type</FormLabel>
                                        <Select defaultValue={initialValues.type} {...field}>
                                            <SelectTrigger className="w-5/6">
                                                <SelectValue placeholder="Select action type" />
                                            </SelectTrigger>
                                            <SelectContent>
                                                <SelectGroup>
                                                    <SelectLabel>Basic Type</SelectLabel>
                                                    {actionTypeOptions.map((option) => (
                                                        <SelectItem key={option.value} value={option.value}>
                                                            {option.label}
                                                        </SelectItem>
                                                    ))}
                                                </SelectGroup>
                                            </SelectContent>
                                        </Select>
                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="command"
                                render={({ field }) => (
                                    <FormItem className='w-5/6'>
                                        <FormLabel className="block text-sm font-medium text-gray-700 ">Command</FormLabel>
                                        <Input
                                            {...field}
                                            type="text"
                                            placeholder="Enter command"
                                            className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                        />

                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="args"
                                render={({ field }) => (
                                    <FormItem className='w-5/6'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Args</FormLabel>
                                        <Input
                                            {...field}
                                            type="text"
                                            placeholder="Enter arguments (comma separated)"
                                            className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                        />

                                    </FormItem>
                                )}
                            />
                        
                        <Button className="col-span-2 w-1/2 mx-auto">Submit</Button>
                        
                    </form>
                </Form>
            </CardContent>
        </Card>
    );
};

export default ActionModify;