import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { App } from 'antd';
import { api } from '../services/api';
import type { CreateGameRequest, UpdateGameRequest } from '../types';

export const GAMES_QUERY_KEY = ['games'];

export function useGames() {
  return useQuery({
    queryKey: GAMES_QUERY_KEY,
    queryFn: () => api.getGames(),
  });
}

export function useGame(id: number | null) {
  return useQuery({
    queryKey: [...GAMES_QUERY_KEY, id],
    queryFn: () => api.getGame(id!),
    enabled: id !== null,
  });
}

export function useCreateGame() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (data: CreateGameRequest) => api.createGame(data),
    onSuccess: (newGame) => {
      queryClient.invalidateQueries({ queryKey: GAMES_QUERY_KEY });
      message.success(`Game "${newGame.name}" created successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to create game';
      message.error(errorMessage);
    },
  });
}

export function useUpdateGame() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: UpdateGameRequest }) =>
      api.updateGame(id, data),
    onSuccess: (updatedGame) => {
      queryClient.invalidateQueries({ queryKey: GAMES_QUERY_KEY });
      queryClient.invalidateQueries({ queryKey: [...GAMES_QUERY_KEY, updatedGame.id] });
      message.success(`Game "${updatedGame.name}" updated successfully`);
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to update game';
      message.error(errorMessage);
    },
  });
}

export function useDeleteGame() {
  const queryClient = useQueryClient();
  const { message } = App.useApp();

  return useMutation({
    mutationFn: (id: number) => api.deleteGame(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: GAMES_QUERY_KEY });
      message.success('Game deleted successfully');
    },
    onError: (error: any) => {
      const errorMessage = error.response?.data?.error || 'Failed to delete game';
      message.error(errorMessage);
    },
  });
}
