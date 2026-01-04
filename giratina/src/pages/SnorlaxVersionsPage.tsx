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
  Tag,
  Badge,
  message,
  Progress,
  App,
} from 'antd';
import {
  PlusOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
  CheckCircleOutlined,
  StarOutlined,
  StarFilled,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import {
  useSnorlaxVersions,
  useSetSnorlaxVersionCurrent,
  useDeleteSnorlaxVersion,
} from '../hooks/useSnorlax';
import { SnorlaxModal } from '../components/SnorlaxModal';
import type { SnorlaxVersion } from '../types';
import { api } from '../services/api';

dayjs.extend(relativeTime);

const { Title } = Typography;

export const SnorlaxVersionsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [searchText, setSearchText] = useState('');
  const [isUploading, setIsUploading] = useState(false);

  const { data: versions = [], isLoading, refetch } = useSnorlaxVersions();
  const setCurrentMutation = useSetSnorlaxVersionCurrent();
  const deleteMutation = useDeleteSnorlaxVersion();

  const { modal: antModal } = App.useApp();

  const filteredVersions = useMemo(() => {
    return versions.filter((version) =>
      version.version.toLowerCase().includes(searchText.toLowerCase()) ||
      version.gcs_path.toLowerCase().includes(searchText.toLowerCase())
    );
  }, [versions, searchText]);

  const handleCreate = () => {
    setModalOpen(true);
  };

  const handleSetCurrent = async (id: number) => {
    await setCurrentMutation.mutateAsync(id);
  };

  const handleDelete = async (id: number) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleModalSubmit = async (values: any) => {
    try {
      const { apkFile, version } = values;

      if (!apkFile) {
        message.error('APK file is required');
        return;
      }

      setIsUploading(true);
      setModalOpen(false);

      // Show progress modal
      const progressModal = antModal.info({
        title: 'Uploading Snorlax APK',
        content: (
          <div>
            <p>Uploading {apkFile.name}...</p>
            <Progress percent={0} status="active" />
            <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
              Please wait while the file is being uploaded.
            </p>
          </div>
        ),
        okButtonProps: { style: { display: 'none' } },
        closable: false,
        maskClosable: false,
      });

      try {
        await api.uploadSnorlaxApk(version, apkFile, (progress) => {
          progressModal.update({
            content: (
              <div>
                <p>Uploading {apkFile.name}...</p>
                <Progress percent={progress} status="active" />
                <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
                  {progress < 100
                    ? 'Please wait while the file is being uploaded.'
                    : 'Processing...'}
                </p>
              </div>
            ),
          });
        });

        progressModal.destroy();
        await refetch();
        message.success(`Snorlax version ${version} created successfully`);
      } catch (error: any) {
        progressModal.destroy();
        const errorMessage = error.response?.data?.error || error.message || 'Failed to upload Snorlax APK';
        message.error(errorMessage);
      } finally {
        setIsUploading(false);
      }
    } catch (error: any) {
      message.error(error.message || 'Failed to upload Snorlax APK');
    }
  };

  const handleModalCancel = () => {
    if (isUploading) {
      antModal.confirm({
        title: 'Upload in progress',
        content: 'Are you sure you want to cancel? The upload will be interrupted.',
        onOk: () => {
          setModalOpen(false);
          setIsUploading(false);
        },
      });
    } else {
      setModalOpen(false);
    }
  };

  const columns: ColumnsType<SnorlaxVersion> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      sorter: (a, b) => a.id - b.id,
    },
    {
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
      width: 150,
      sorter: (a, b) => a.version.localeCompare(b.version),
      render: (version: string, record: SnorlaxVersion) => (
        <Space>
          <strong>{version}</strong>
          {record.is_current && (
            <Tag icon={<CheckCircleOutlined />} color="success">
              CURRENT
            </Tag>
          )}
        </Space>
      ),
    },
    {
      title: 'GCS Path',
      dataIndex: 'gcs_path',
      key: 'gcs_path',
      render: (path: string) => (
        <Tooltip title={`Full path: ${path}/Snorlax.apk`}>
          <code style={{ fontSize: 12 }}>{path}</code>
        </Tooltip>
      ),
    },
    {
      title: 'Release Date',
      dataIndex: 'release_date',
      key: 'release_date',
      width: 180,
      render: (date: string) => (
        <Tooltip title={dayjs(date).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(date).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.release_date).unix() - dayjs(b.release_date).unix(),
    },
    {
      title: 'Created',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 180,
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
      width: 150,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          {!record.is_current && (
            <Tooltip title="Set as Current">
              <Button
                type="text"
                icon={<StarOutlined />}
                onClick={() => handleSetCurrent(record.id)}
                loading={setCurrentMutation.isPending}
                size="small"
                style={{ color: '#f59e0b' }}
              />
            </Tooltip>
          )}
          {record.is_current && (
            <Tooltip title="Currently Active">
              <StarFilled style={{ color: '#f59e0b', fontSize: 16 }} />
            </Tooltip>
          )}
          <Popconfirm
            title="Delete version?"
            description={
              record.is_current
                ? "Cannot delete the current version. Set another version as current first."
                : "Are you sure you want to delete this Snorlax version?"
            }
            onConfirm={() => handleDelete(record.id)}
            okText="Delete"
            okButtonProps={{ danger: true, disabled: record.is_current }}
            cancelText="Cancel"
            disabled={record.is_current}
          >
            <Tooltip title={record.is_current ? "Cannot delete current version" : "Delete"}>
              <Button
                type="text"
                danger
                icon={<DeleteOutlined />}
                size="small"
                disabled={record.is_current}
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const currentVersion = versions.find(v => v.is_current);

  return (
    <div>
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }}>
        <div>
          <Title level={2} style={{ margin: 0, marginBottom: 8 }}>
            Snorlax Versions
          </Title>
          {currentVersion && (
            <Badge
              status="processing"
              text={`Current version: ${currentVersion.version}`}
              style={{ fontSize: 14 }}
            />
          )}
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
        >
          Create Version
        </Button>
      </Flex>

      <Card>
        <Flex gap="middle" style={{ marginBottom: 16 }} wrap="wrap">
          <Input
            placeholder="Search versions..."
            prefix={<SearchOutlined />}
            allowClear
            style={{ width: 300 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
          />
          <Tooltip title="Refresh">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
            />
          </Tooltip>
        </Flex>

        <Table
          columns={columns}
          dataSource={filteredVersions}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `Total ${total} versions`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50'],
          }}
          rowClassName={(record) =>
            record.is_current ? 'current-version-row' : ''
          }
        />
      </Card>

      <SnorlaxModal
        open={modalOpen}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={isUploading}
      />

      <style>{`
        .current-version-row {
          background-color: rgba(234, 88, 12, 0.05) !important;
        }
        .current-version-row:hover > td {
          background-color: rgba(234, 88, 12, 0.08) !important;
        }
      `}</style>
    </div>
  );
};
