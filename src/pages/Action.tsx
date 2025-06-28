import React, { useState } from 'react';
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"

import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Tooltip, TooltipContent, TooltipTrigger, } from "@/components/ui/tooltip"
import { Input } from '@/components/ui/input';
import { Card, CardContent, } from "@/components/ui/card"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue, } from "@/components/ui/select"
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from "@/components/ui/skeleton"

import { open_file_select_dialog, simplifyPath } from '@/utils';
import { create_action } from '@/api/modules/action';
import type { Action } from '@/types';
const formSchema = z.object({
    name: z.string().min(2, {
        message: "action name must be at least 2 characters.",
    }),
    type: z.string(),
    command: z.string(),
    args: z.string(),
    desc: z.string(),
    wait: z.coerce.number(),
})

const ActionModify: React.FC = () => {
    const [isFileType, setIsFileType] = useState<boolean>(true);
    const [isUrlType, setIsUrlType] = useState<boolean>(false);
    const [isCommandType, setIsCommandType] = useState<boolean>(false);


    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            type: "open_file",
            command: "",
            args: "",
            desc: "",
            wait: 0,
        },
    })

    // const initialValues: Action = {
    //     name: "",
    //     desc: "",
    //     command: "",
    //     args: [],
    //     type: "open_file",
    //     wait: 1,
    //     retry: 0,
    // };

    const actionTypeOptions = [
        { value: "open_file", label: "Open File" },
        { value: "open_dir", label: "Open Directory" },
        { value: "exec_command", label: "Execute Command" },
        { value: "open_url", label: "Open URL" },
    ];
    const onSubmit = (values: z.infer<typeof formSchema>) => {
        console.log(values)
        const args = values.args.split(",")
        const actionData:Action = {...values,args}
        create_action(actionData)
    }
    const handleSelectFile = async () => {
        const isFile = form.getValues("type") === "open_file"
        const path = await open_file_select_dialog(isFile);
        form.setValue("command", path)
        console.log(path);
    }

    return (
        <Card className="action-modify" >
            <CardContent>
                <Form {...form} >
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                    >
                        <div className='grid gap-4 grid-cols-2 mb-1'>
                            <FormField
                                control={form.control}
                                name="name"
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
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
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Type</FormLabel>
                                        <Select {...field} onValueChange={(e) => {
                                            field.onChange(e)
                                            setIsFileType(e === "open_file" || e === "open_dir")
                                            setIsUrlType(e === "open_url")
                                            setIsCommandType(e === "exec_command")
                                        }}>
                                            <FormControl>
                                                <SelectTrigger className="w-5/6">
                                                    <SelectValue placeholder="Select action type" />
                                                </SelectTrigger>
                                            </FormControl>
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
                        </div>
                        <FormField
                            control={form.control}
                            name="desc"
                            render={({ field }) => (
                                <FormItem className='w-5/6 mb-2'>
                                    <FormLabel className="block text-sm font-medium text-gray-700">Desc</FormLabel>
                                    <FormControl>
                                        <Textarea
                                            {...field}
                                            placeholder="Enter action desc"
                                            className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 resize-none"
                                        />
                                    </FormControl>
                                    <p className="text-muted-foreground text-sm">
                                        Describe the action.
                                    </p>
                                </FormItem>
                            )}
                        />
                        <div className='grid gap-4 grid-cols-2 '>
                            <FormField
                                name="command"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700 ">{isUrlType ? "Url" : isCommandType ? "Command" : "Path"}</FormLabel>
                                        <FormControl>
                                            {!isFileType ? <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter command"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            /> : field.value != '' ? 
                                                <Tooltip>
                                                    <TooltipTrigger asChild>
                                                        <Badge className='justify-start'>
                                                            {simplifyPath(field.value)}
                                                        </Badge>
                                                    </TooltipTrigger>
                                                    <TooltipContent >
                                                        {field.value}
                                                    </TooltipContent>
                                                </Tooltip>: <Skeleton className="h-5 w-[100px]"/>
                                            }
                                        </FormControl>
                                    </FormItem>
                                )}
                            />
                            {isCommandType ? <FormField
                                name="args"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Args</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter arguments (comma separated)"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />
                                        </FormControl>
                                    </FormItem>
                                )}
                            /> : isFileType ? <Button
                                variant="ghost"
                                className='w-5/6 h-full cursor-pointer'
                                onClick={(e)=>{e.preventDefault();handleSelectFile()}}>
                                <span className="material-symbols-outlined">upload_file</span>
                            </Button> : null
                            }
                        </div>
                        <div className="grid gap-4 grid-cols-2">
                            <FormField
                                name="wait"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Wait</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="number"
                                                placeholder="Enter wait time"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />
                                        </FormControl>
                                        <FormMessage/>
                                    </FormItem>
                                )}
                            />
                        </div>
                        <div className="divider">
                            <Button className="col-span-2 w-1/2 mx-auto cursor-pointer">Submit</Button>
                        </div>


                    </form>
                </Form>
            </CardContent>
        </Card>
    );
};

export default ActionModify;