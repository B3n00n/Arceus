import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { Assignment, CreateAssignmentRequest, UpdateAssignmentRequest } from '../types';

export const ASSIGNMENTS_QUERY_KEY = ['assignments'];

export function useAssignments() {
  return useQuery({
    queryKey: ASSIGNMENTS_QUERY_KEY,
    queryFn: () => api.getAssignments(),
  });
}

export function useCreateAssignment() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (data: CreateAssignmentRequest) => api.createAssignment(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ASSIGNMENTS_QUERY_KEY });
      message.success('Assignment created successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create assignment';
      message.error(errorMessage);
    },
  });
}

export function useUpdateAssignment() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: UpdateAssignmentRequest }) =>
      api.updateAssignment(id, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ASSIGNMENTS_QUERY_KEY });
      message.success('Assignment updated successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to update assignment';
      message.error(errorMessage);
    },
  });
}

export function useDeleteAssignment() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.deleteAssignment(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ASSIGNMENTS_QUERY_KEY });
      message.success('Assignment deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete assignment';
      message.error(errorMessage);
    },
  });
}
