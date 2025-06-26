import React from 'react';
import { useForm } from "react-hook-form"
import {Form,FormField,FormItem } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { invoke } from '@tauri-apps/api/core';

import type { Action } from "../types";
// import SubmitActionButton from '../components/Invoke/SubmitAction';
import {simplifyPath} from '@/utils'
import { useNavigate } from 'react-router-dom';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { ActionSelect } from '@/components/Action';


const ActionModify: React.FC = () => {
    const form = useForm()
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
        { value: "exec_command", label: "Execute Command" },
        { value: "open_dir", label: "Open Directory" },
        { value: "open_url", label: "Open URL" },
    ];
    const ActionTypeSelect = ()=>{
        return (
            <Select defaultValue={initialValues.type}>
                <SelectTrigger className="w-56">
                    <SelectValue placeholder="Select action type" />
                </SelectTrigger>
                <SelectContent>
                    <SelectGroup>
                        {actionTypeOptions.map((option) => (
                            <SelectItem key={option.value} value={option.value}>
                                {option.label}
                            </SelectItem>
                        ))}
                    </SelectGroup>
                </SelectContent>
            </Select>
        );

    }

    return (
        <div className="action-modify" >
            <Form {...form} >
                <FormField
                name="name"
                render={({ field }) => (
                    <FormItem>
                        <label className="block text-sm font-medium text-gray-700">Action Name</label>
                        <Input
                            {...field}
                            type="text"
                            placeholder="Enter action name"
                            className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                        />
                    </FormItem>
                )}
                />
                <FormField
                name="desc"
                render={({ field }) => (
                    <FormItem>
                        <label className="block text-sm font-medium text-gray-700">Action Description</label>
                        <Select defaultValue={initialValues.type} {...field}>
                            <SelectTrigger className="w-56">
                                <SelectValue placeholder="Select action type" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
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


            </Form>
        </div>
    );
};

export default ActionModify;