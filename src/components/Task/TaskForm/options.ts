import type { Period } from "@/types";

const reminderOptions = [
  { value: 'ontime', label: 'On Time', offset: 0 },
  { value: '5min', label: 'Before 5 Minutes', offset: 5 * 60 * 1000 },
  { value: '15min', label: 'Before 15 Minutes', offset: 15 * 60 * 1000 },
  { value: '30min', label: 'Before 30 Minutes', offset: 30 * 60 * 1000 },
  { value: '1hour', label: 'Before 1 Hour', offset: 60 * 60 * 1000 },
  { value: '2hour', label: 'Before 2 Hours', offset: 2 * 60 * 60 * 1000 },
  { value: '1day', label: 'Before 1 Day', offset: 24 * 60 * 60 * 1000 },
  { value: '3day', label: 'Before 3 Days', offset: 3 * 24 * 60 * 60 * 1000 },
  { value: '1week', label: 'Before 1 Week', offset: 7 * 24 * 60 * 60 * 1000 },
] as const;

const periodOptions = [
  { value: 0 as Period, label: 'Period OnStart', description: 'Period OnStart Description' }, // Period.OnStart
  { value: 100 as Period, label: 'Period OnceStarted', description: 'Period OnceStarted Description' }, // Period.OnceStarted
  { value: 1 as Period, label: 'Period Daily', description: 'Period Daily Description' }, // Period.Daily
  { value: 7 as Period, label: 'Period Weekly', description: 'Period Weekly Description' }, // Period.Weekly
  { value: 30 as Period, label: 'Period Monthly', description: 'Period Monthly Description' }, // Period.Monthly
] as const;

export { reminderOptions, periodOptions };