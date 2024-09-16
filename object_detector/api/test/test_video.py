# coding: utf-8

"""
    Djangolang

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.video import Video

class TestVideo(unittest.TestCase):
    """Video unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Video:
        """Test Video
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Video`
        """
        model = Video()
        if include_optional:
            return Video(
                camera_id = '',
                camera_id_object = openapi_client.models.camera.Camera(
                    created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    id = '', 
                    last_seen = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    name = '', 
                    referenced_by_detection_camera_id_objects = [
                        openapi_client.models.detection.Detection(
                            bounding_box = [
                                openapi_client.models.detection_bounding_box_inner.Detection_bounding_box_inner(
                                    x = 1.337, 
                                    y = 1.337, )
                                ], 
                            camera_id = '', 
                            camera_id_object = openapi_client.models.camera.Camera(
                                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                id = '', 
                                last_seen = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                name = '', 
                                referenced_by_video_camera_id_objects = [
                                    openapi_client.models.video.Video(
                                        camera_id = '', 
                                        camera_id_object = , 
                                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        detection_summary = null, 
                                        duration = 56, 
                                        ended_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        file_name = '', 
                                        file_size = 1.337, 
                                        id = '', 
                                        object_detector_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        object_tracker_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        referenced_by_detection_video_id_objects = [
                                            openapi_client.models.detection.Detection(
                                                camera_id = '', 
                                                centroid = openapi_client.models.detection_bounding_box_inner.Detection_bounding_box_inner(
                                                    x = 1.337, 
                                                    y = 1.337, ), 
                                                class_id = 56, 
                                                class_name = '', 
                                                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                id = '', 
                                                score = 1.337, 
                                                seen_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                video_id = '', 
                                                video_id_object = openapi_client.models.video.Video(
                                                    camera_id = '', 
                                                    created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                    deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                    detection_summary = null, 
                                                    duration = 56, 
                                                    ended_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                    file_name = '', 
                                                    file_size = 1.337, 
                                                    id = '', 
                                                    object_detector_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                    object_tracker_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                    started_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                    status = '', 
                                                    thumbnail_name = '', 
                                                    updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ), )
                                            ], 
                                        started_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        status = '', 
                                        thumbnail_name = '', 
                                        updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                                    ], 
                                segment_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                stream_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                stream_url = '', 
                                updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ), 
                            centroid = , 
                            class_id = 56, 
                            class_name = '', 
                            created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            id = '', 
                            score = 1.337, 
                            seen_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            video_id = '', 
                            video_id_object = , )
                        ], 
                    referenced_by_video_camera_id_objects = [
                        
                        ], 
                    segment_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    stream_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                    stream_url = '', 
                    updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ),
                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                detection_summary = None,
                duration = 56,
                ended_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                file_name = '',
                file_size = 1.337,
                id = '',
                object_detector_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                object_tracker_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                referenced_by_detection_video_id_objects = [
                    openapi_client.models.detection.Detection(
                        bounding_box = [
                            openapi_client.models.detection_bounding_box_inner.Detection_bounding_box_inner(
                                x = 1.337, 
                                y = 1.337, )
                            ], 
                        camera_id = '', 
                        camera_id_object = openapi_client.models.camera.Camera(
                            created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            id = '', 
                            last_seen = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            name = '', 
                            referenced_by_video_camera_id_objects = [
                                openapi_client.models.video.Video(
                                    camera_id = '', 
                                    camera_id_object = openapi_client.models.camera.Camera(
                                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        id = '', 
                                        last_seen = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        name = '', 
                                        segment_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        stream_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                        stream_url = '', 
                                        updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ), 
                                    created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                    deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                    detection_summary = null, 
                                    duration = 56, 
                                    ended_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                    file_name = '', 
                                    file_size = 1.337, 
                                    id = '', 
                                    object_detector_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                    object_tracker_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                    referenced_by_detection_video_id_objects = [
                                        openapi_client.models.detection.Detection(
                                            camera_id = '', 
                                            camera_id_object = , 
                                            centroid = openapi_client.models.detection_bounding_box_inner.Detection_bounding_box_inner(
                                                x = 1.337, 
                                                y = 1.337, ), 
                                            class_id = 56, 
                                            class_name = '', 
                                            created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                            deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                            id = '', 
                                            score = 1.337, 
                                            seen_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                            updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                            video_id = '', 
                                            video_id_object = openapi_client.models.video.Video(
                                                camera_id = '', 
                                                camera_id_object = , 
                                                created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                detection_summary = null, 
                                                duration = 56, 
                                                ended_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                file_name = '', 
                                                file_size = 1.337, 
                                                id = '', 
                                                object_detector_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                object_tracker_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                referenced_by_detection_video_id_objects = , 
                                                started_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                                status = '', 
                                                thumbnail_name = '', 
                                                updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ), )
                                        ], 
                                    started_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                                    status = '', 
                                    thumbnail_name = '', 
                                    updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), )
                                ], 
                            segment_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            stream_producer_claimed_until = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                            stream_url = '', 
                            updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), ), 
                        centroid = , 
                        class_id = 56, 
                        class_name = '', 
                        created_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        deleted_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        id = '', 
                        score = 1.337, 
                        seen_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'), 
                        video_id = '', 
                        video_id_object = , )
                    ],
                started_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f'),
                status = '',
                thumbnail_name = '',
                updated_at = datetime.datetime.strptime('2013-10-20 19:20:30.00', '%Y-%m-%d %H:%M:%S.%f')
            )
        else:
            return Video(
        )
        """

    def testVideo(self):
        """Test Video"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
