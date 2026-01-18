import { useState, useMemo } from 'react';
import {
  Typography,
  Button,
  Table,
  Space,
  Input,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { useGames, useCreateGame, useUpdateGame, useDeleteGame } from '../hooks/useGames';
import { GameModal } from '../components/GameModal';
import type { Game } from '../types';
import { api } from '../services/api';

dayjs.extend(relativeTime);

const { Title } = Typography;

export const GamesPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedGame, setSelectedGame] = useState<Game | undefined>();
  const [searchText, setSearchText] = useState('');

  const { data: games = [], isLoading, refetch } = useGames();
  const createMutation = useCreateGame();
  const updateMutation = useUpdateGame();
  const deleteMutation = useDeleteGame();

  const filteredGames = useMemo(() => {
    return games.filter((game) =>
      game.name.toLowerCase().includes(searchText.toLowerCase())
    );
  }, [games, searchText]);

  const handleCreate = () => {
    setModalMode('create');
    setSelectedGame(undefined);
    setModalOpen(true);
  };

  const handleEdit = (game: Game) => {
    setModalMode('edit');
    setSelectedGame(game);
    setModalOpen(true);
  };

  const handleDelete = async (id: number) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleModalSubmit = async (values: any) => {
    try {
      if (modalMode === 'create') {
        const { backgroundFile, ...gameData } = values;
        const newGame = await createMutation.mutateAsync(gameData);

        if (backgroundFile && newGame?.id) {
          await api.uploadGameBackground(newGame.id, backgroundFile);
        }
      } else if (selectedGame) {
        await updateMutation.mutateAsync({
          id: selectedGame.id,
          data: values,
        });
      }
      setModalOpen(false);
      setSelectedGame(undefined);
    } catch {
      // Error handling is done in the mutation hooks
    }
  };

  const handleModalCancel = () => {
    setModalOpen(false);
    setSelectedGame(undefined);
  };

  const columns: ColumnsType<Game> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      sorter: (a, b) => a.id - b.id,
      render: (id: number) => (
        <span style={{ color: '#94a3b8', fontWeight: 500, fontSize: 13 }}>#{id}</span>
      ),
    },
    {
      title: 'Image',
      dataIndex: 'background_url',
      key: 'background_url',
      width: 100,
      render: (url: string | undefined) =>
        url ? (
          <img
            src={url}
            alt="Game background"
            style={{
              width: 80,
              height: 48,
              objectFit: 'cover',
              borderRadius: 6,
              border: '1px solid #475569',
              boxShadow: '0 1px 3px rgba(0, 0, 0, 0.3)'
            }}
          />
        ) : (
          <div style={{
            width: 80,
            height: 48,
            backgroundColor: '#0f172a',
            borderRadius: 6,
            border: '1px solid #475569',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 11,
            color: '#64748b'
          }}>
            No image
          </div>
        ),
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
      render: (name: string) => <strong>{name}</strong>,
    },
    {
      title: 'Created',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 200,
      render: (created: string) => (
        <Tooltip title={dayjs(created).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(created).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.created_at).unix() - dayjs(b.created_at).unix(),
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 140,
      fixed: 'right',
      align: 'center',
      render: (_, record) => (
        <Space size={8}>
          <Tooltip title="Edit Game">
            <Button
              type="default"
              icon={<EditOutlined />}
              onClick={() => handleEdit(record)}
              size="middle"
              style={{
                borderRadius: 6,
                borderColor: '#475569',
              }}
            />
          </Tooltip>
          <Popconfirm
            title="Delete game?"
            description="This will also delete all versions and assignments for this game."
            onConfirm={() => handleDelete(record.id)}
            okText="Delete"
            okButtonProps={{ danger: true }}
            cancelText="Cancel"
          >
            <Tooltip title="Delete Game">
              <Button
                danger
                icon={<DeleteOutlined />}
                size="middle"
                style={{
                  borderRadius: 6,
                }}
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header Section */}
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, fontSize: 28, fontWeight: 600 }}>
            Games
          </Title>
          <div style={{ marginTop: 8, color: '#94a3b8', fontSize: 14 }}>
            {filteredGames.length} game{filteredGames.length !== 1 ? 's' : ''} total
          </div>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
          style={{ minHeight: 42 }}
        >
          Create Game
        </Button>
      </Flex>

      {/* Main Content Card */}
      <Card
        style={{
          borderRadius: 12,
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.2), 0 2px 4px -2px rgb(0 0 0 / 0.2)',
        }}
      >
        {/* Search Bar */}
        <Flex gap={12} style={{ marginBottom: 20 }} wrap="wrap" align="center">
          <Input
            placeholder="Search games by name..."
            prefix={<SearchOutlined style={{ color: '#64748b' }} />}
            allowClear
            style={{ maxWidth: 400, flex: 1 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            size="large"
          />
          <Tooltip title="Refresh Data">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
              size="large"
            >
              Refresh
            </Button>
          </Tooltip>
        </Flex>

        {/* Data Table */}
        <Table
          columns={columns}
          dataSource={filteredGames}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `${total} game${total !== 1 ? 's' : ''} total`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50', '100'],
            style: { marginTop: 16 },
          }}
          scroll={{ x: 800 }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>

      <GameModal
        open={modalOpen}
        mode={modalMode}
        game={selectedGame}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={createMutation.isPending || updateMutation.isPending}
      />
    </div>
  );
};
