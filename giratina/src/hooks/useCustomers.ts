import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { CreateCustomerRequest, UpdateCustomerRequest } from '../types';

export const CUSTOMERS_QUERY_KEY = ['customers'];

export function useCustomers() {
  return useQuery({
    queryKey: CUSTOMERS_QUERY_KEY,
    queryFn: () => api.getCustomers(),
  });
}

export function useCustomer(id: number | null) {
  return useQuery({
    queryKey: [...CUSTOMERS_QUERY_KEY, id],
    queryFn: () => api.getCustomer(id!),
    enabled: id !== null,
  });
}

export function useCreateCustomer() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (data: CreateCustomerRequest) => api.createCustomer(data),
    onSuccess: (newCustomer) => {
      queryClient.invalidateQueries({ queryKey: CUSTOMERS_QUERY_KEY });
      message.success(`Customer "${newCustomer.name}" created successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create customer';
      message.error(errorMessage);
    },
  });
}

export function useUpdateCustomer() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: UpdateCustomerRequest }) =>
      api.updateCustomer(id, data),
    onSuccess: (updatedCustomer) => {
      queryClient.invalidateQueries({ queryKey: CUSTOMERS_QUERY_KEY });
      queryClient.invalidateQueries({ queryKey: [...CUSTOMERS_QUERY_KEY, updatedCustomer.id] });
      // Also invalidate arcades since arcade assignments may have changed
      queryClient.invalidateQueries({ queryKey: ['arcades'] });
      message.success(`Customer "${updatedCustomer.name}" updated successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to update customer';
      message.error(errorMessage);
    },
  });
}

export function useDeleteCustomer() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.deleteCustomer(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CUSTOMERS_QUERY_KEY });
      // Also invalidate arcades since arcade assignments may have changed
      queryClient.invalidateQueries({ queryKey: ['arcades'] });
      message.success('Customer deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete customer';
      message.error(errorMessage);
    },
  });
}
