# coding: utf-8

"""
    Djangolang

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.patch_custom0_request import PatchCustom0Request

class TestPatchCustom0Request(unittest.TestCase):
    """PatchCustom0Request unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> PatchCustom0Request:
        """Test PatchCustom0Request
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `PatchCustom0Request`
        """
        model = PatchCustom0Request()
        if include_optional:
            return PatchCustom0Request(
                claim_duration_seconds = 1.337
            )
        else:
            return PatchCustom0Request(
                claim_duration_seconds = 1.337,
        )
        """

    def testPatchCustom0Request(self):
        """Test PatchCustom0Request"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()